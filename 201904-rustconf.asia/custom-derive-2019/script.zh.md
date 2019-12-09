# (Re:) 从零开始编写一个Custom Derive

（根据我在RustCon Asia 2019上的演讲所备的原稿整理而成）

Servo的样式系统中对custom derive有着广泛的使用。我想从Servo样式系统中的实例出发，介绍一下custom derive的应用，以及如何从零开始开发一个custom derive。

## Custom Derive的应用

我们可以看这么一个例子，这是Servo的类型系统中一个比较有代表性的类型，可以看到它有13个derive，除了Rust自己的`Clone`、`Copy`、`Debug`、`PartialEq`以及依赖中提供的`MallocSizeOf`以外， 剩下8个都是样式系统中专用的。

```rust
#[derive(
    Animate,
    Clone,
    ComputeSquaredDistance,
    Copy,
    Debug,
    MallocSizeOf,
    PartialEq,
    Parse,
    SpecifiedValueInfo,
    ToAnimatedValue,
    ToAnimatedZero,
    ToComputedValue,
    ToCss,
)]
/// Either `<color>` or `auto`.
pub enum ColorOrAuto<C> {
    /// A `<color>
    Color(C),
    /// `auto`
    Auto,
}
```

如果我们这里不用custom derive而是手写出这8个的代码，它大概长这样：

<details>

```rust
use cssparser::{Parser, Token};
use parser::{Parse, ParserContext};
use std::fmt::{self, Write};
use style_traits::{CssWriter, KeywordsCollectFn, ParseError, SpecifiedValueInfo, ToCss};
use values::animated::{Animate, Procedure, ToAnimatedValue, ToAnimatedZero};
use values::computed::{Context, ToComputedValue};
use values::distance::{ComputeSquaredDistance, SquaredDistance};

impl<C: Animate> Animate for ColorOrAuto<C> {
    fn animate(&self, other: &Self, procedure: Procedure) -> Result<Self, ()> {
        match (self, other) {
            (&ColorOrAuto::Color(ref this), &ColorOrAuto::Color(ref other)) => {
                this.animate(other, procedure).map(ColorOrAuto::Color)
            }
            (&ColorOrAuto::Auto, &ColorOrAuto::Auto) => {
                Ok(ColorOrAuto::Auto)
            }
            _ => Err(())
        }
    }
}

impl<C: ComputeSquaredDistance> ComputeSquaredDistance for ColorOrAuto<C> {
    fn compute_squared_distance(&self, other: &Self) -> Result<SquaredDistance, ()> {
        match (self, other) {
            (&ColorOrAuto::Color(ref this), &ColorOrAuto::Color(ref other)) => {
                this.compute_squared_distance(other)
            }
            (&ColorOrAuto::Auto, &ColorOrAuto::Auto) => {
                Ok(SquaredDistance::from_sqrt(0.))
            }
            _ => Err(())
        }
    }
}

impl<C: Parse> Parse for ColorOrAuto<C> {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        if let Ok(v) = input.try(|i| C::parse(context, i)) {
            return Ok(ColorOrAuto::Color(v));
        }
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        match_ignore_ascii_case! { &ident,
            "auto" => Ok(ColorOrAuto::Auto),
            _ => Err(location.new_unexpected_token_error(Token::Ident(ident.clone()))),
        }
    }
}

impl<C: SpecifiedValueInfo> SpecifiedValueInfo for ColorOrAuto<C> {
    const SUPPORTED_TYPES: u8 = C::SUPPORTED_TYPES;
    fn collect_completion_keywords(f: KeywordsCollectFn) {
        C::collect_completion_keywords(f);
        f(&["auto"]);
    }
}

impl<C: ToAnimatedValue> ToAnimatedValue for ColorOrAuto<C> {
    type AnimatedValue = ColorOrAuto<C::AnimatedValue>;
    fn to_animated_value(self) -> Self::AnimatedValue {
        match self {
            ColorOrAuto::Color(c) => ColorOrAuto::Color(c.to_animated_value()),
            ColorOrAuto::Auto => ColorOrAuto::Auto,
        }
    }
    fn from_animated_value(animated: Self::AnimatedValue) -> Self {
        match animated {
            ColorOrAuto::Color(c) => ColorOrAuto::Color(C::from_animated_value(c)),
            ColorOrAuto::Auto => ColorOrAuto::Auto,
        }
    }
}

impl<C: ToAnimatedZero> ToAnimatedZero for ColorOrAuto<C> {
    fn to_animated_zero(&self) -> Result<Self, ()> {
        match self {
            ColorOrAuto::Color(c) => c.to_animated_zero().map(ColorOrAuto::Color),
            ColorOrAuto::Auto => Ok(ColorOrAuto::Auto),
        }
    }
}

impl<C: ToComputedValue> ToComputedValue for ColorOrAuto<C> {
    type ComputedValue = ColorOrAuto<C::ComputedValue>;
    fn to_computed_value(&self, context: &Context) -> Self::ComputedValue {
        match self {
            ColorOrAuto::Color(c) => ColorOrAuto::Color(c.to_computed_value(context)),
            ColorOrAuto::Auto => ColorOrAuto::Auto,
        }
    }
    fn from_computed_value(computed: &Self::ComputedValue) -> Self {
        match computed {
            ColorOrAuto::Color(c) => ColorOrAuto::Color(C::from_computed_value(c)),
            ColorOrAuto::Auto => ColorOrAuto::Auto,
        }
    }
}

impl<C: ToCss> ToCss for ColorOrAuto<C> {
    fn to_css<W: fmt::Write>(&self, dest: &mut CssWriter<W>) -> fmt::Result {
        match self {
            ColorOrAuto::Color(c) => c.to_css(dest),
            ColorOrAuto::Auto => dest.write_str("auto"),
        }
    }
}
```

</details>

为什么样式系统中需要用到这么多custom derive呢？原因很简单，因为CSS很复杂。

举例来说，CSS里面一个值在不同的阶段需要不同的表达方式，开发者编写CSS的时候写的是文本形式，我们需要把它解析成指定值（specified value），而后指定值要按照一定的规则计算并应用到各个元素上成为计算值（computed value），除此之外因为值在动画过渡的过程中可能需要不同的精度以及不同的取值范围，因而还有一个单独的动画值（animated value）。有这么多不同的形式，我们自然也需要在这些形式之间进行转换。

将数据在不同的形式之间转换也许是custom derive最合适的应用场景了。Rust生态圈里非常有名的[Serde](https://crates.io/crates/serde)便是custom derive在这个场景下的一个经典应用：在结构化数据和一些通用的序列化格式之间进行转换。

在Stylo中除了数据转换以外，custom derive还被用在一些递归计算，如计算两个值之间的距离，还有某些简单的编译期反射上。

## 如何编写Custom derive

那么应该如何编写custom derive呢？

我在第一次写custom derive之前，觉得那些代码非常抽象，难以理解。那时候别人让我review这些代码，我都是拒绝的。我想custom derive的代码看起来抽象是有原因的。正常的函数，你给一个输入，它给一个输出，你可以很直观地对比它的输出结果与你的期望，也可以很方便地查看中间过程。而对于custom derive来说，它输出的是一段程序，而你能对比的往往只有它所输出的程序的运行结果。增加了一层间接性，自然会增加理解代码的难度。因此我认为，要编写和理解custom derive，最重要的就是你要对你想生成的程序有一个清晰的概念。

### 基础代码

举例来说，Servo里面有一个trait叫`ToCss`，是用来将某种类型转换到CSS文本形式的。这个trait的声明大体是这样：

```rust
pub trait ToCss {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write;
}
```

需要注意的是custom derive需要放在一个独立的crate里，并且需要在crate的`Cargo.toml`里写上

```toml
[lib]
proc-macro = true
```

然后我们就先在它的`lib.rs`里写下这个custom derive的最外层代码：

```rust
extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro_derive(ToCss)]
pub fn derive_to_css(input: TokenStream) -> TokenStream {
    unimplemented!()
}
```

可以看到custom derive输入的是一个`TokenStream`输出也是一个`TokenStream`，这没什么特别有趣的地方。需要注意的是即使在Rust 2018里，那个`extern crate`目前也依然是必须的，关于解决掉它的方式也[还在讨论之中](https://github.com/rust-lang/rust/issues/57288)。

CSS中最常见的一类值是关键字，比如说`white-space`可以有`normal`、`nowrap`、`pre`、`pre-wrap`、`pre-line`等值，在Rust里面我们显然应该以枚举的形式来表示，像是这样：

```rust
pub enum WhiteSpace {
    Normal,
    Nowrap,
    Pre,
    PreWrap,
    PreLine,
}
```

如果我们要为它手动实现`ToCss`，大概应该长这样：

```rust
impl ToCss for WhiteSpace {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            WhiteSpace::Normal => dest.write_str("normal"),
            WhiteSpace::Nowrap => dest.write_str("nowrap"),
            WhiteSpace::Pre => dest.write_str("pre"),
            WhiteSpace::PreWrap => dest.write_str("pre-wrap"),
            WhiteSpace::PreLine => dest.write_str("pre-line"),
        }
    }
}
```

观察这个实现的代码，我们会发现它有一些比较固定的与类型本身无关的结构，我们可以先把这些结构写下来

```rust
#[recursion_limit = "128"]

use proc_macro::TokenStream;
use quote::quote;

TokenStream::from(quote! {
    impl style_traits::ToCss for /* ??? */ {
        fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
        where
        		W: std::fmt::Write,
        {
            match self {
                /* ??? */
            }
        }
    }
})
```

这里我们用到了一个宏 `quote`，这个宏是来自[同名的crate](https://crates.io/crates/quote)，是开发custom derive中使用的两个最重要的crate之一，它可以让你像这样直接写出你要生成的代码，省去很多麻烦。但因为`quote`可能产生很深的宏递归，所以很多时候需要在文件的开头加上`#![recursion_limit = "128"]`。

可以看到这里我们在`quote!`之外又包了一层`TokenStream::from`。这是因为`quote`产生的虽然也是`TokenStream`，但却是来自[`proc-marco2`](https://crates.io/crates/proc-macro2)这个crate的。`proc-macro2`的`TokenStream`可以`Into`到`proc_macro`的`TokenStream`中去。为什么有了自带的`proc_macro`还需要`proc-macro2`呢？我也没有能完全说清楚的自信，应该有一些历史和实践上的原因，有兴趣的可以自己研究ww

此外你们或许注意到，这里所有的类型和trait我们用的都是完整路径，因为你不知道这个宏会被放在什么地方，所以保险起见用上完整路径比较好。

### 解析和代码生成

`TokenStream`如其名字所述，就是一串符号而已，最好要有什么东西能帮我们将这串符号解析成方便实用的形式。这里就要用到[`syn`](https://crates.io/crates/syn)这个crate了。这个crate实现了一个Rust代码的解析器，用来把一串符号按照Rust的语法解析成语法树。

参照文档，我们可以写出下面这样的代码：

```rust
use heck::KebabCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

let input = parse_macro_input!(input as DeriveInput);
let name = input.ident;
let match_body = match input.data {
    Data::Enum(data) => {
        data.variants.into_iter().flat_map(|variant| {
            match variant.fields {
                Fields::Unit => {
                    let ident = variant.ident;
                    let value = ident.to_string().to_kebab_case();
                    quote! {
                        #name::#ident => std::fmt::Write::write_str(dest, #value),
                    }
                }
                _ => panic!("unsupported variant fields"),
            }
        }).collect::<TokenStream2>()
    }
    _ => panic!("unsupported data structure"),
};
TokenStream::from(quote! {
    impl style_traits::ToCss for #name {
        fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
        where
        		W: std::fmt::Write,
        {
            match self {
                #match_body
            }
        }
    }
})
```

这里我们先将原来代码中的占位内容替换成两个变量，在`quote`里直接用井号加上变量名就可以了。接下来要做的就只是在前面计算出这两个变量的内容。

为此我们先将输入数据解析成`DeriveInput`的格式，然后假定其为简单的枚举类型，将枚举每个分支转换成一条`match`分支的代码。

这里我们用到了`heck`这个crate来转换大小写写法，但这不是特别重要。

另外可以看到里面的`write_str`我们也用了展开的写法，原因和之前一样，因为不知道生成代码的位置，所以要用最保险的写法。这样我们就写出了一个最基本的custom derive了。对于类似`white-space`这样使用关键字的CSS属性就可以直接derive出`ToCss`的实现了。

### 代码的拆分

这里用了两个不同的`TokenStream`看起来让人不是很舒服。结合这个问题，加上很多时候你会编写不止一个custom derive，通常custom derive的主要逻辑部分都会拆到另外的模块里，比如上面的例子通常会这样拆，在`lib.rs`里写上包装代码：

```rust
extern crate proc_macro;
use proc_macro::TokenStream;

mod to_css;

#[proc_macro_derive(ToCss)]
pub fn derive_to_css(input: TokenStream) -> TokenStream {
    to_css::derive(syn::parse_macro_input!(input)).into()
}
```

然后在独立的模块`to_css.rs`里写上：

```rust
use heck::KebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Variant};

pub fn derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let match_body: TokenStream = match input.data {
        Data::Enum(data) => data
            .variants
            .into_iter()
            .flat_map(|variant| derive_variant(&name, variant))
            .collect(),
        _ => panic!("unsupported data structure"),
    };
    quote! {
        impl style_traits::ToCss for #name {
            fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
            where
                W: std::fmt::Write,
            {
                match self {
                    #match_body
                }
            }
        }
    }
}

fn derive_variant(name: &Ident, variant: Variant) -> TokenStream {
    match variant.fields {
        Fields::Unit => {
            let ident = variant.ident;
            let value = ident.to_string().to_kebab_case();
            quote! {
                #name::#ident => std::fmt::Write::write_str(dest, #value),
            }
        }
        _ => panic!("unsupported variant fields"),
    }
}
```

注意这个模块里我们就只接触`proc-macro2`的`TokenStream`了，所有和`proc_macro::TokenStream`的交互都交给了`lib.rs`，这样我们也就可以直接返回`quote!`的结果而不需要另外包装了。

### 带字段的枚举

我们上面只处理了不带任何字段的枚举，但在现实中有很多带字段的。

还是CSS的例子，比如`initial-letters`这个属性就支持`normal`或者一个数字和一个整数作为它的值，我们可能会将其定义为这种形式：

```rust
pub enum InitialLetters {
    Normal,
    Values(f32, i32),
}
```

这时候我们手写的`ToCss`大概是这样

```rust
match self {
    InitialLetters::Normal => dest.write_str("normal"),
    InitialLetters::Values(v1, v2) => write!(dest, "{} {}", v1, v2),
}
```

但这种写法显然是对我们这里的状况非常特化的，即`ToCss`输出字符串，而且这里的两个字段正好都实现了`Display`。如果我们不是要输出字符串，或者我们要处理不是`Display`的类型要怎么办？

通常我们的策略是，假定一个类型里每个字段的类型都实现了同一个trait，然后我们在custom derive生成的代码里递归调用trait的相应方法就可以了。所以我们要先给`f32`和`i32`实现`ToCss`，这样我们可以把上面实现中的对两个类型特异的部分换成通用的递归调用：

```rust
match self {
    InitialLetters::Normal => dest.write_str("normal"),
    InitialLetters::Values(v1, v2) => {
        v1.to_css(dest)?;
        dest.write_char(' ')?;
        v2.to_css(dest)?;
        Ok(())
    }
}
```

于是我们就对这个部分应该怎么写有了概念，剩下的就简单了，我们在之前`Fields::Unit`的分支下面加上一个新的分支

```rust
Fields::Unnamed(fields) => {
    let bindings = &(0..fields.unnamed.len())
        .map(|i| Ident::new(&format!("v{}", i), Span::call_site()))
        .collect::<Vec<_>>();
    let is_first = iter::once(true).chain(iter::repeat(false));
    quote! {
        #name::#ident(#(#bindings),*) => {
            #(
                if !#is_first {
                    std::fmt::Write::write_char(dest, ' ')?;
                }
                style_traits::ToCss::to_css(#bindings, dest)?;
            )*
            Ok(())
        }
    }
}
```

这里首先生成了与无名字段数量同样多的绑定变量的变量名。我们用了`v`加数字的形式作为名称然后新建一个`Ident`表示其为一个标识符。注意我们不能直接用字符串，因为字符串转换为token以后会变成字符串字面量的token，前面我们在`Unit`的分支就有利用过这一点。如果我们需要标识符的话就要用上`Ident`。

另外还可以看到`quote`里面也可以像`macro_rules`的宏定义里面那样支持重复，只不过用的也是井号。这里我们用了一个小技巧，结合标准库自带的几个迭代器用来插入中间的空格。

支持了无名字段，其实命名字段也是类似的了，无名字段和命名字段之间的差异，无非一个用小括号一个用大括号，以及一个没有名字所以需要我们生成绑定变量名，另一个直接用字段名就好了。我们可以把共通的中间部分提取出来

```rust
fn derive_fields_body(bindings: &[Ident]) -> TokenStream {
    let is_first = iter::once(true).chain(iter::repeat(false));
    quote! {
        #(
            if !#is_first {
                std::fmt::Write::write_char(dest, ' ')?;
            }
            style_traits::ToCss::to_css(#bindings, dest)?;
        )*
        Ok(())
    }
}
```

然后就简单了

```rust
Fields::Named(fields) => {
    let field_names = fields
        .named
        .into_iter()
        .map(|field| field.ident.unwrap())
        .collect::<Vec<_>>();
    let body = derive_fields_body(&field_names);
    quote! {
        #name::#ident { #(#field_names),* } => {
            #body
        }
    }
}
```

这样我们就可以把那个`unsupported variant fields`的panic分支给删掉了，因为我们支持了所有可能的枚举分支类型。

### 结构体

也许你很早就发现了，其实枚举的三种分支类型也是结构体的三种类型，那么我们是不是可以直接复用这些代码给结构体呢？

我们来看这么一个结构体类型：

```rust
pub struct CounterPair {
    name: CustomIdent,
    value: i32,
}
```

要实现这个结构体的`ToCss`我们可能会这么写

```rust
self.name.to_css(dest)?;
dest.write_char(' ')?;
self.value.to_css(dest)?;
Ok(())
```

但这么写的话上面的代码似乎复用起来会稍微麻烦一点？其实虽然可能不常用，但对于结构体我们也可以用`match`来匹配，于是我们可以把其中的代码写成这样的形式：

```rust
match self {
    CounterPair { name, value } => {
        name.to_css(dest)?;
        dest.write_char(' ')?;
        value.to_css(dest)?;
        Ok(())
    }
}
```

这种形式是不是很熟悉了？跟前面枚举的写法几乎如出一辙。于是我们可以将`derive_variant`改名叫`derive_fields`并做一点简单的修改：

```rust
fn derive_fields(ident: &Ident, pat: TokenStream, fields: Fields) -> TokenStream {
    // ...
}
```

然后就可以将其同时用于枚举和结构体了

```rust
let match_body = match input.data {
    Data::Struct(data) => derive_fields(&name, quote!(#name), data.fields),
    Data::Enum(data) => data
        .variants
        .into_iter()
        .flat_map(|variant| {
            let ident = variant.ident;
            derive_fields(&ident, quote!(#name::#ident), variant.fields)
        })
        .collect(),
    _ => panic!("unsupported data structure"),
};
```

### 泛型

在更复杂的情况下我们还会遇到泛型类型，比如说这个枚举

```rust
pub enum ColorOrAuto<C> {
    Color(C),
    Auto,
}
```

这里的泛型参数`C`是某个代表颜色的类型。

与处理字段的策略类似，对于泛型我们一般也假定所有类型参数都有实现同一个trait。这里手动为其实现`ToCss`我们大概要这么写：

```rust
impl<C> ToCss for ColorOrAuto<C>
where
    C: ToCss,
{
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match self {
            ColorOrAuto::Color(c) => c.to_css(dest),
            ColorOrAuto::Auto => dest.write_str("auto"),
        }
    }
}
```

可以看到函数内部的代码没有差别，主要的差别是`impl`块的声明部分了。显然这里我们需要做的是在`quote`的代码里也加上泛型的部分，我们可以这么做：

```rust
if !input.generics.params.is_empty() {
    let mut where_clause = input.generics.where_clause.take();
    let predicates = &mut where_clause.get_or_insert(parse_quote!(where)).predicates;
    for param in input.generics.type_params() {
        let ident = &param.ident;
        predicates.push(parse_quote!(#ident: style_traits::ToCss));
    }
    input.generics.where_clause = where_clause;
}
let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
quote! {
    impl#impl_generics style_traits::ToCss for #name#ty_generics
    #where_clause
    {
        // same as before
    }
}
```

这段代码看过去很多东西，但其实解析起来也并不复杂。

我们首先把`where`子句给拆出来，然后为每一个类型参数加上一个实现`ToCss`的约束，再把添加好的`where`子句装回去。为什么还要装回去呢？往下看可以发现`syn`非常贴心地为我们准备了一个`split_for_impl`的方法可以轻松地生成我们需要的部分，最后直接把这些部分加到`quote`里就可以了。

从这个代码可以看到我们保留了原来类型声明所带有的所有类型约束，这在目前的Rust依然是必须的。在不远的将来如果隐含约束实现了，这部分的代码也可以简化一些吧。

### 属性

有的时候我们可能会需要微调trait的一些行为，这时候就可以用到属性。

比如说CSS的`transform-style`属性有一个值是`preserve-3d`，但`heck`并不能将`Preserve3d`转换到我们需要的形式，这时候我们就希望能用一个属性来解决，比如这样

```rust
#[derive(ToCss)]
pub enum TransformStyle {
    Flat,
    #[css(keyword = "preserve-3d")]
    Preserve3d,
}
```

为了实现这样的功能，我们可以使用一个叫[`darling`](https://crates.io/crates/darling)的crate，可以将属性解析存放到一个结构体中。

对于这里的需求，我们可以写这样一个结构体

```rust
#[derive(Default, FromVariant)]
#[darling(attributes(css), default)]
struct CssVariantAttrs {
    keyword: Option<String>,
}
```

这里的derive的`FromVariant`表示可以从枚举的分支数据中解析属性信息。与此类似的还有`FromField`、`FromDeriveInput`等可以解析字段或者整个类型的属性信息。`attributes(css)`表示这个结构体对应的是`css`这个属性。这里应该还是比较直观的。

然后就在之前得到枚举分支的地方调用`from_variant`来解析属性信息：

```rust
let attrs = CssVariantAttrs::from_variant(&variant)
		.expect("failed to parse variant attributes");
let ident = variant.ident;
derive_fields(&ident, quote!(#name::#ident), variant.fields, Some(attrs))
```

并在生成代码的地方根据这个属性的值生成出对应的代码即可

```rust
let value = attrs
    .and_then(|attrs| attrs.keyword)
    .unwrap_or_else(|| ident.to_string().to_kebab_case());
quote! {
    #pat => std::fmt::Write::write_str(dest, #value),
}
```

最后我们需要在`lib.rs`里告诉Rust这个custom derive可以解析`css`这个属性，只要这样：

```rust
#[proc_macro_derive(ToCss, attributes(css))]
pub fn derive_to_css(input: TokenStream) -> TokenStream {
    to_css::derive(syn::parse_macro_input!(input)).into()
}
```

### synstructure

前面我们一步步分析了这个custom derive的实现，但你或许发现了在这么多代码里其实大多数代码都在处理枚举、结构体、分支、泛型之类的东西，这些代码看起来应该在不同的custom derive实现里都可以通用的。那么是否有什么现成的工具可以替我们处理这些东西呢？答案是肯定的，那就是[`synstructure`](https://crates.io/crates/synstructure)。

`synstructure`基本上是一个custom derive的工具包和框架。我们上面的代码如果用它的话只要这么简单就可以了：

```rust
fn derive_to_css(input: Structure) -> TokenStream {
    let body = input.each_variant(|vi| {
        let bindings = vi.bindings();
        if bindings.is_empty() {
            let value = vi.ast().ident.to_string().to_kebab_case();
            return quote! {
                std::fmt::Write::write_str(dest, #value)
            };
        }
        let is_first = iter::once(true).chain(iter::repeat(false));
        quote! {
            #(
                if !#is_first {
                    std::fmt::Write::write_char(dest, ' ')?;
                }
                style_traits::ToCss::to_css(#bindings, dest)?;
            )*
            Ok(())
        }
    });
    input.gen_impl(quote! {
        gen impl style_traits::ToCss for @Self {
            fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
            where
                W: std::fmt::Write,
            {
                match self {
                    #body
                }
            }
        }
    })
}

decl_derive!([ToCss] => derive_to_css);
```

`synstructure`将结构体和枚举分支都抽象成了一个分支信息的类型，我们只需要对每个分支生成其执行的代码，其他外围代码就都由`synstructure`的`each_variant`来代劳了，泛型之类的处理也交给了它的`gen_impl`，我们只需要专注写最核心的代码就可以了。

既然`synstructure`这么方便为什么我不一开始就介绍呢？因为我之前也提到了，写custom derive最重要的是你要对你生成的代码有概念。`synstructure`虽然好用，但我觉得它隐藏了太多代码生成的细节，对一开始的理解和编写custom derive并不是很好。但当你理解了custom derive以后，`synstructure`会成为一个非常趁手的工具。

眼尖的人可能注意到了，在用`synstructure`的代码里面对属性的支持消失了。这是因为`synstructure`和`darling`在枚举分支属性的支持上交互性不太好，这一点我想未来还有改进的空间。

## 总结

完整的代码可以在我相应的[代码仓库](https://github.com/upsuper/custom-derive-2019)中找到，有兴趣的话可以进一步查看。

我想声明的是，这里看到的代码都是我为这个演讲从头写出来的，货真价实的从零开始。Servo里相应的代码跟这里有许多区别，区别的一个主要原因当然是实际应用中的需求更复杂，毕竟一开始也说了，CSS很复杂。除此之外，Servo里的那些代码写成的时间也要早许多，特别是过去一年Rust的快速发展和生态圈的完善让很多代码可以有更简化的写法，但已经写成的代码没有特别的必要当然也就没人去改写了。