#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Handle Handle;

typedef struct {
	uint8_t x;
	uint8_t y;
} Vector;

typedef struct {
	uint8_t min;
	uint8_t max;
} MinMax;

typedef struct {
	uint32_t seed;
	size_t rooms_count;
	Vector rooms_min_size;
	Vector rooms_max_size;
	MinMax rooms_spacing;
	MinMax path_extension;
} Config;

extern Handle* map_create(Config *config);
extern void map_destroy(Handle *handle);
extern Vector map_size(Handle *handle);
extern char* map_as_string(Handle *handle);
extern char* map_as_bytes(Handle *handle);
extern Config* get_config();

void print_ascii(char* map)
{
	printf("%s\n", map);
	//printf("Generated map (ASCII): %s\n", map);
}

void print_bytes(char* map, Vector size)
{
	printf("Generated map (bytes):\n");

	for (int y = 0; y < size.y; y++)
	{
		for (int x = 0; x < size.x; x++)
		{
			printf("%i", map[x + y * size.x]);
		}

		printf("\n");
	}
}

Config* create_config(int seed)
{
	Config* config = get_config();

	config->seed = seed;
	config->rooms_count = 7;
	config->rooms_min_size = (Vector) { 4, 4 };
	config->rooms_max_size = (Vector) { 7, 7 };
	config->rooms_spacing = (MinMax) { 3, 5 };
	config->path_extension = (MinMax) { 2, 4 };

	return config;
}

void print_map(Handle* handle, bool draw_ascii, bool draw_bytes)
{
	if (draw_ascii)
	{
		print_ascii(map_as_string(handle));
	}

	if (draw_bytes)
	{
		print_bytes(map_as_bytes(handle), map_size(handle));
	}

	printf("\n");
}

void draw_map(uint32_t seed)
{
	Config* config = create_config(seed);
	Handle* handle = map_create(config);

	printf("Map %u", seed);
	print_map(handle, true, false);

	map_destroy(handle);
}

int main()
{
	draw_map(42);

	return 0;
}
