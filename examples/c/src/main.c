#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

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

int main()
{
	Config* config = get_config();

	config->seed = 42;
	config->rooms_count = 11;
	config->rooms_min_size = (Vector) { 4, 4 };
	config->rooms_max_size = (Vector) { 7, 7 };
	config->rooms_spacing = (MinMax) { 3, 5 };
	config->path_extension = (MinMax) { 2, 4 };

	// TODO - Add config check

	printf("Map seed: %u\n", config->seed);
	printf("Number of rooms: %zu\n", config->rooms_count);

    Handle* handle = map_create(config);
	Vector size = map_size(handle);
	char* map_ascii = map_as_string(handle);
	char* map_bytes = map_as_bytes(handle);

	printf("Map size: %ux%u\n", size.x, size.y);
	printf("\n");
	printf("Generated map (ASCII): %s\n", map_ascii);
	printf("\n");
	printf("Generated map (bytes):\n");

	for (int y = 0; y < size.y; y++)
	{
		for (int x = 0; x < size.x; x++)
		{
			printf("%i", map_bytes[x + y * size.x]);
		}

		printf("\n");
	}

	map_destroy(handle);

	return 0;
}
