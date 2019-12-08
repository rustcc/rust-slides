void fun2()
{
    int m = 1; 
    int n = 2;
}

int* fun1()
{
    int *p;
    int q = 0;
    p = &q;
    return p; // bug
}

int main()
{
    int *a, b;

    a = fun1();
    *a = 10;
    fun2();
    b = *a;
}

