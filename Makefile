CC		= gcc
CFLAGS	= -Wall -Werror -Wextra
LDLIBS	= -lm

ifeq ($(BUILD), Debug)
CFLAGS	+= -g -O0 -fsanitize=address -D DEBUG
SUFFIX	= .debug.o
NAME	= dungen.debug
else
SUFFIX	= .o
NAME	= dungen
endif

ARCH	= $(shell uname -m)
OS		= $(shell uname -s)

SRC_PREFIX	= ./examples/c/src/
OBJ_PREFIX	= ./obj/

SRC	=	$(shell ls $(SRC_PREFIX)*.c)

ifeq ($(BUILD), Debug)
	SRC	+= $(shell ls $(SRC_PREFIX)debug/*.c)
endif

OBJ = $(subst $(SRC_PREFIX), , $(SRC:.c=$(SUFFIX)))
OBJ := $(addprefix $(OBJ_PREFIX), $(OBJ))

.PHONY: all clean fclean re debug

all: $(NAME)

debug:
	@$(MAKE) BUILD=Debug all

$(NAME): $(OBJ) target/debug/libdungeon_generator.a
	@echo "making $@"
	$(CC) -o $(NAME) $(OBJ) $(CFLAGS) $(LDLIBS) target/debug/libdungeon_generator.a

target/debug/libdungeon_generator.a: lib

lib:
	@echo "making $@"
	@cargo build

$(OBJ_PREFIX)%$(SUFFIX): $(SRC_PREFIX)%.c
	@echo "making $<"
	@mkdir -p $(OBJ_PREFIX)
	@mkdir -p $(OBJ_PREFIX)debug/
	@$(CC) -o $@ -c $< $(CFLAGS)

clean:
	/bin/rm -rf $(OBJ_PREFIX)

fclean: clean
	/bin/rm -f $(NAME)

re: fclean all
