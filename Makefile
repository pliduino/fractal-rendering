CC = g++
CFLAGS = -Wall
SRC = cpp-generation/src
OUTPUT = cpp-generation/out
MAIN = main
INCLUDE = cpp-generation/include cpp-generation/pybind11/include

FILES = $(shell find cpp-generation/src/**/*.cpp)
OBJECTS = $(FILES:.cpp=.o)


$(shell mkdir -p $(OUTPUT))

%.cpp.o:
	$(CC) $(CFLAGS) $< -c -o $@

all: $(OBJECTS)
	$(CC) -I$(INCLUDE) $(CFLAGS) -o $(OUTPUT)/$(MAIN) $(OBJECTS)

run: all
	./$(OUTPUT)/$(MAIN)

clean:
	rm -f $(OBJECTS) $(OUTPUT)/$(MAIN)