SYSTEM:= $(shell uname -s)

CP= cp -i
INSTALL= install -m755 --strip
GO= go
MD= mkdir -p
RM= rm -f

SOURCES= $(wildcard *.go) $(wildcard **/*.go)
PREFIX?= $(HOME)/.local/share
TARGET= $(PREFIX)/dockermon


#-------------------------------------------------------------------------------
.PHONY: clean check test


all: dockermon


clean:
	$(RM) dockermon


check:
	@type lazydocker >/dev/null || printf 'Remember to install \e[32;1mlazydocker\e[0m\n'

install: dockermon $(TARGET) check
	$(INSTALL) $< $(TARGET)
	@$(CP) call-docker docker.png $(TARGET)/ || true


test:
	@echo 'Feel free to write some tests :-)'


dockermon: $(SOURCES)
	$(GO) build


$(TARGET):
	$(MD) $@
