OS := $(shell uname)

LEX= flex
YACC= bison -dy
RM= rm -f

ifeq ($(OS),Darwin)
	CC= clang
	LD= clang
else
	CC= gcc
	LD= gcc
endif

YSRC= $(wildcard *.y)
LSRC= $(wildcard *.l)
YCSRC= $(YSRC:%.y=%.tab.c)
LCSRC= $(LSRC:%.l=%.lex.c)
YHDR= $(YCSRC:%.c=%.h)
OBJS= $(YCSRC:%.c=%.o) $(LCSRC:%.c=%.o)
TARGET= $(YSRC:%.y=%)


#-------------------------------------------------------------------------------
.PHONY: clean mrproper test


all: $(TARGET)


clean:
	$(RM) $(LCSRC) $(YCSRC) $(YHDR) $(OBJS)


test: $(TARGET) hello.txt
	./$?


mrproper: clean
	$(RM) $(TARGET)


$(TARGET): $(OBJS)
	$(LD) $? -lm -o $@


.c.o:
	$(CC) -c $< -o $@


%.lex.c: %.l
	$(LEX) -o $@ $<

%.tab.c: %.y
	$(YACC) -o $@ $<
