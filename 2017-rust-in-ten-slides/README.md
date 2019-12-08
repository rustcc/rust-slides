# Rust in Ten Slides

This is an experiment to produce short, useful documentation for the Rust
programming language.

Here's the idea: take a concept in Rust, and make ten slides about it. Ten
slides is short enough to be easy to make and easy to consume, but long
enough to get some kind of point across.

If this is successful, I'd like to upstream it into the main Rust docs.

This project is inspired by [A very brief intro to
Rust](https://github.com/ashleygwilliams/a-very-brief-intro-to-rust). After
seeing students write code after only looking at these slides, I think
expanding on this idea might bear some interesting fruit!

## How it works

1. Copy `template.html` into a new file, name it after the topic of your
   presentation; for example, `strings.html`.
2. Slides are written in Markdown inside the file using [Remark](https://remarkjs.com/#1),
   check that link for examples of the syntax.
3. Open the file in your web browser to see the presentation in action!
4. Modify `index.html` to include your new presentation.
5. Send me a pull request!

## License

Everything is licensed under MIT/Apache2, just like Rust.