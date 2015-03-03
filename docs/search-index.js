var searchIndex = {};
searchIndex['clap'] = {"items":[[0,"","clap","A simply library for parsing command line arguments when writing\n command line and console applications."],[3,"ArgMatches","","Used to get information about the arguments that\nwhere supplied to the program at runtime."],[12,"name","","",0],[12,"flags","","",0],[12,"opts","","",0],[12,"positionals","","",0],[3,"Arg","","The abstract representation of a command line argument used by the consumer of the library.\n "],[12,"name","","The unique name of the argument, required",1],[12,"short","","The short version (i.e. single character) of the argument, no preceding `-`\n**NOTE:** `short` is mutually exclusive with `index`",1],[12,"long","","The long version of the flag (i.e. word) without the preceding `--`\n**NOTE:** `long` is mutually exclusive with `index`",1],[12,"help","","The string of text that will displayed to the user when the application's\n`help` text is displayed",1],[12,"required","","If this is a required by default when using the command line program\ni.e. a configuration file that's required for the program to function\n**NOTE:** required by default means, it is required *until* mutually\nexclusive arguments are evaluated.",1],[12,"takes_value","","Determines if this argument is an option, vice a flag or positional and\nis mutually exclusive with `index` and `multiple`",1],[12,"index","","The index of the argument. `index` is mutually exclusive with `takes_value`\nand `multiple`",1],[12,"multiple","","Determines if multiple instances of the same flag are allowed. `multiple`\nis mutually exclusive with `index` and `takes_value`.\nI.e. `-v -v -v` or `-vvv`",1],[12,"blacklist","","A list of names for other arguments that *may not* be used with this flag",1],[12,"requires","","A list of names of other arguments that are *required* to be used when\nthis flag is used",1],[3,"App","","Used to create a representation of the program and all possible command line arguments\n for parsing at runtime."],[12,"name","","The name displayed to the user when showing version and help/usage information",2],[12,"author","","A string of author(s) if desired. Displayed when showing help/usage information",2],[12,"version","","The version displayed to the user",2],[12,"about","","A brief explaination of the program that gets displayed to the user when shown help/usage information",2],[11,"new","","Creates a new instance of an application requiring a name (such as the binary). Will be displayed\nto the user when they print version or help and usage information.",2],[11,"author","","Sets a string of author(s)",2],[11,"about","","Sets a string briefly describing what the program does",2],[11,"version","","Sets a string of the version number",2],[11,"arg","","Adds an argument to the list of valid possibilties",2],[11,"args","","Adds arguments to the list of valid possibilties",2],[11,"get_matches","","",2],[11,"new","","Creates a new instance of `ArgMatches`. This ins't called directly, but\nthrough the `.get_matches()` method of `App`",0],[11,"value_of","","Gets the value of a specific option or positional argument (i.e. an argument that takes\nan additional value at runtime). If the option wasn't present at runtime\nit returns `None`",0],[11,"is_present","","Checks if a flag was argument was supplied at runtime. **DOES NOT** work for\noption or positional arguments (use `.value_of()` instead)",0],[11,"occurrences_of","","Checks the number of occurrences of a flag at runtime.",0],[11,"new","","Creates a new instace of `Arg` using a unique string name.\nThe name will be used by the library consumer to get information about\nwhether or not the argument was used at runtime. ",1],[11,"short","","Sets the short version of the argument without the preceding `-`.",1],[11,"long","","Sets the long version of the argument without the preceding `--`.",1],[11,"help","","Sets the help text of the argument that will be displayed to the user\nwhen they print the usage/help information. ",1],[11,"required","","Sets whether or not the argument is required by default. Required by\ndefault means it is required, when no other mutually exlusive rules have\nbeen evaluated. Mutually exclusive rules take precedence over being required\nby default.",1],[11,"mutually_excludes","","Sets a mutually exclusive argument by name. I.e. when using this argument,\nthe following argument can't be present.",1],[11,"mutually_excludes_all","","Sets a mutually exclusive arguments by names. I.e. when using this argument,\n the following argument can't be present.",1],[11,"requires","","Sets an argument by name that is required when this one is presnet I.e. when\nusing this argument, the following argument *must* be present.",1],[11,"requires_all","","Sets arguments by names that are required when this one is presnet I.e. when\n using this argument, the following arguments *must* be present.",1],[11,"takes_value","","Specifies that the argument takes an additional value at run time.\n \n**NOTE:** When setting this to `true` the `name` of the argument\nwill be used when printing the help/usage information to the user. ",1],[11,"index","","Specifies the index of a positional argument starting at 1.\n \n**NOTE:** When setting this,  any `short` or `long` values you set\nare ignored as positional arguments cannot have a `short` or `long`.\nAlso, the name will be used when printing the help/usage information \nto the user. ",1],[11,"multiple","","Specifies if the flag may appear more than once such as for multiple debugging\nlevels (as an example). `-ddd` for three levels of debugging, or `-d -d -d`. \nWhen this is set to `true` you recieve the number of occurances the user supplied\nof a particular flag at runtime.\n \n**NOTE:** When setting this,  any `takes_value` or `index` values you set\nare ignored as flags cannot have a values or an `index`.",1]],"paths":[[3,"ArgMatches"],[3,"Arg"],[3,"App"]]};
initSearch(searchIndex);
