%{
    #include <stdlib.h>
    #include <stdio.h>
    #include "stcalc.tab.h"

    void yyerror(const char *);
%}

%%

[ \s\t\r\n] ;   // whitespaces

0\.?                    { yylval.dValue = atof(yytext); return NUMBER; }
[1-9][0-9]\.[0-9]+      { yylval.dValue = atof(yytext); return NUMBER; }
[1-9][0-9]*\.?/[^0-9]   { yylval.dValue = atof(yytext); return NUMBER; }
\.[0-9]+                { yylval.dValue = atof(yytext); return NUMBER; }

[-+*/^] { yylval.cValue = *yytext; return OPERATOR; }
=       { return PRINT_NUMBER; }
!       { return PRINT_CHAR; }

[^0-9.+*/^=! \t\r\n-].* {
    size_t length = strlen(yytext) + 14;
    char tmp[length];
    sprintf(tmp, "invalid token: %s", yytext);
    yyerror(tmp);
}

%%

int yywrap(void) {
    return 1;
}
