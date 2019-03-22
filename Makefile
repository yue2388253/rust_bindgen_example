CC = g++
CFLAGS := -Wall -fpic

MKDIR_P = mkdir -p
OUT_DIR = target

OBJS = $(OUT_DIR)/hello.o

SRCS = src/hello.c

TARGET = $(OUT_DIR)/libhello.a

all: $(TARGET)

$(TARGET): $(OBJS)
	rm -f $@
	ar cr $@ $(OBJS)
	rm -f $(OBJS)

$(OBJS) : $(SRCS)
	$(MKDIR_P) $(OUT_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(TARGET) $(OBJS)

.PHONY: clean all