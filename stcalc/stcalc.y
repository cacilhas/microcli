%{
    #include <stdlib.h>
    #include <stdio.h>
    #include <math.h>
    #include <string.h>

    #define STACKSIZE 1024

    FILE *yyin;
    unsigned char not_interactive;
    int yylex(void);
    void yyerror(const char *);
    double stack[STACKSIZE];
    size_t position = 0;
%}

%union {
    int    iValue;
    char   cValue;
    double dValue;
}

%token <dValue> NUMBER
%token <cValue> OPERATOR
%token PRINT_NUMBER PRINT_CHAR

%%

program
    : program statement
    |
    ;

statement
    : NUMBER    {
                    if (position >= STACKSIZE)
                        yyerror("stack is full");
                    stack[position++] = $1;
                }
    | OPERATOR  {
                    if (position < 2)
                        yyerror("insufficient data");
                    --position;
                    if ($1 == '+')
                        stack[position - 1] += stack[position];
                    else if ($1 == '-')
                        stack[position - 1] -= stack[position];
                    else if ($1 == '*')
                        stack[position - 1] *= stack[position];
                    else if ($1 == '/')
                        stack[position - 1] /= stack[position];
                    else if ($1 == '^')
                        stack[position - 1] = pow(stack[position - 1],
                                                  stack[position]);
                }
    | PRINT_NUMBER  {
                        if (position == 0)
                            yyerror("nothing to show");
                        printf(" %g", stack[position - 1]);
                    }
    | PRINT_CHAR    {
                        if (position == 0)
                            yyerror("nothing to show");
                        unsigned char aux = (unsigned char) stack[position - 1] % 255;
                        const char c = (const char) aux;
                        printf("%c", c);
                    }
    ;

%%

int main(int argc, char *argv[]) {
    not_interactive = (argc >= 2);
    if (not_interactive)
        yyin = fopen(argv[1], "r");

    memset(stack, 0, STACKSIZE);
    yyparse();

    if (not_interactive)
        fclose(yyin);
    return EXIT_SUCCESS;
}


void yyerror(const char *s) {
    fprintf(stderr, "%s\n", s);
    if (not_interactive)
        exit(EXIT_FAILURE);
}
