We use clang-format to keep the coding style consistent. A .clang-format file is shipped within the source. Feel free to use it!

For now, you can use it like this:

`clang-format -i <file to format>`

See `man clang-format`. Don't use the -i flag if you'd like to preview your changes first. 

If you need to format the entire project:

`find . -iname *.h -o -iname *.c | xargs clang-format -i`

(To start with a fresh .clang-format file try: https://zed0.co.uk/clang-form    at-configurator/)
