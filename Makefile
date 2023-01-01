CC = g++
CFLAGS = -Wall
SRC = src
OUTPUT = out
MAIN = main
INCLUDE = include

FILES = $(shell find ./src/**/*.cpp)
OBJECTS = $(FILES:.cpp=.o)


$(shell mkdir -p $(OUTPUT))

%.cpp.o:
	$(CC) $(CFLAGS) $< -c -o $@

all: $(OBJECTS)
	$(CC) -I$(INCLUDE) $(CFLAGS) -o $(OUTPUT)/$(MAIN) $(OBJECTS) -lSDL2main -lSDL2

run: all
	./$(OUTPUT)/$(MAIN)

clean:
	rm -f $(OBJECTS) $(OUTPUT)/$(MAIN)