extern crate libc;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::vec::IntoIter;

use args::{ ArgMatches, Arg, OptArg, FlagArg, PosArg, SubCommand };

/// Used to create a representation of the program and all possible command line arguments
/// for parsing at runtime.
///
///
/// Stores a list of all posisble arguments, as well as information displayed to the user such as
/// help and versioning information.
///
/// # Example
///
/// ```no_run
/// # use clap::{App, Arg};
/// let myprog = App::new("myprog")
///                   .author("Me, me@mail.com")
///                      .version("1.0.2")
///                   .about("Explains in brief what the program does")
///                   .arg(
///                            Arg::new("in_file").index(1)
///                        // Add other possible command line argument options here...
///                    )
///                   .get_matches();
///
/// // Your pogram logic starts here...
/// ```
pub struct App {
    // The name displayed to the user when showing version and help/usage information
    name: &'static str,
    // A string of author(s) if desired. Displayed when showing help/usage information
    author: Option<&'static str>,
    // The version displayed to the user
    version: Option<&'static str>,
    // A brief explaination of the program that gets displayed to the user when shown help/usage information
    about: Option<&'static str>,
    flags: HashMap<&'static str, FlagArg>,
    opts: HashMap<&'static str, OptArg>,
    positionals_idx: BTreeMap<u8, PosArg>,
    subcommands: HashMap<&'static str, Box<App>>,
    needs_long_help: bool,
    needs_long_version: bool,
    needs_short_help: bool,
    needs_short_version: bool,
    needs_subcmd_help: bool,
    required: HashSet<&'static str>,
    arg_list: HashSet<&'static str>,
    short_list: HashSet<char>,
    long_list: HashSet<&'static str>,
    blacklist: HashSet<&'static str>,
    usage_str: Option<&'static str>,
    bin_name: Option<String>

}

impl App {
    /// Creates a new instance of an application requiring a name (such as the binary). Will be displayed
    /// to the user when they print version or help and usage information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let prog = App::new("myprog")
    /// # .get_matches();
    /// ```
    pub fn new(n: &'static str) -> App {
        App {
            name: n,
            author: None,
            about: None,
            version: None,
            flags: HashMap::new(),
            opts: HashMap::new(),
            positionals_idx: BTreeMap::new(),
            subcommands: HashMap::new(),
            // positionals_name: HashMap::new(),
            needs_long_version: true,
            needs_long_help: true,
            needs_short_help: true,
            needs_subcmd_help: true,
            needs_short_version: true,
            required: HashSet::new(), 
            arg_list: HashSet::new(),
            short_list: HashSet::new(),
            long_list: HashSet::new(),
            usage_str: None,
            blacklist: HashSet::new(),
            bin_name: None,
        }
    }

    /// Sets a string of author(s)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .author("Kevin <kbknapp@gmail.com>")
    /// # .get_matches();
    /// ```
    pub fn author(mut self, a: &'static str) -> App {
        self.author = Some(a);
        self
    }

    /// Sets a string briefly describing what the program does
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .about("Does really amazing things to great people")
    /// # .get_matches();
    /// ```
    pub fn about(mut self, a: &'static str) -> App {
        self.about = Some(a);
        self
    }

    /// Sets a string of the version number
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .version("v0.1.24")
    /// # .get_matches();
    /// ```
    pub fn version(mut self, v: &'static str) -> App  {
        self.version = Some(v);
        self
    }
    
    /// Sets a custom usage string to over-ride the one auto-generated by `clap`
    /// 
    /// *NOTE:* You do not need to specify the "USAGE: " portion, as that will 
    /// still be applied by `clap`, you only need to specify the portion starting
    /// with the binary name. 
    /// 
    /// *NOTE:* This will not replace the entire help message, only the portion
    /// showing the usage.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .usage("myapp [-clDas] <some_file>")
    /// # .get_matches();
    /// ```
    pub fn usage(mut self, u: &'static str) -> App {
        self.usage_str = Some(u);
        self
    }

    /// Adds an argument to the list of valid possibilties
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .arg(Arg::new("config")
    ///                .short("c")
    ///             // Additional argument configuration goes here...
    /// )
    /// # .get_matches();
    /// ```
    pub fn arg(mut self, a: Arg) -> App {
        if self.arg_list.contains(a.name) {
            panic!("Argument name must be unique, \"{}\" is already in use", a.name);
        } else {
            self.arg_list.insert(a.name);
        }
        if let Some(ref s) = a.short {
            if self.short_list.contains(s) {
                panic!("Argument short must be unique, -{} is already in use", s);
            } else {
                self.short_list.insert(*s);
            }
        }
        if let Some(ref l) = a.long {
            if self.long_list.contains(l) {
                panic!("Argument long must be unique, --{} is already in use", l);
            } else {
                self.long_list.insert(l);
            }
        }
        if a.required {
            self.required.insert(a.name);
        }
        if let Some(i) = a.index {
            self.positionals_idx.insert(i, PosArg {
                name: a.name,
                index: i,
                required: a.required,
                blacklist: a.blacklist,
                requires: a.requires,
                help: a.help,
                value: None
            });
        } else if a.takes_value {
            if a.short == None && a.long == None {
                panic!("An argument that takes a value must have either a .short() or .long() [or both] assigned");
            }
            self.opts.insert(a.name, OptArg {
                name: a.name,
                short: a.short,
                long: a.long,
                multiple: a.multiple,
                blacklist: a.blacklist,
                help: a.help,
                requires: a.requires,
                required: a.required,
                values: vec![]
            });
        } else {
            if let Some(ref l) = a.long {
                if *l == "help" {
                    self.needs_long_help = false;
                } else if *l == "version" {
                    self.needs_long_version = false;
                }
            }
            if let Some(ref s) = a.short {
                if *s == 'h' {
                    self.needs_short_help = false;
                } else if *s == 'v' {
                    self.needs_short_version = false;
                }
            }
            if a.short == None && a.long == None {
                panic!("A flag argument must have either a .short() or .long() [or both] assigned");
            }
            // Flags can't be required
            if self.required.contains(a.name) {
                self.required.remove(a.name);
            }
            self.flags.insert(a.name, FlagArg{
                name: a.name,
                short: a.short,
                long: a.long,
                help: a.help,
                blacklist: a.blacklist,
                multiple: a.multiple,
                requires: a.requires,
                occurrences: 1
            });
        }
        self
    }

    /// Adds multiple arguments to the list of valid possibilties
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let app = App::new("myprog")
    /// .args( vec![Arg::new("config").short("c"),
    ///                Arg::new("debug").short("d")])
    /// # .get_matches();
    /// ```
    pub fn args(mut self, args: Vec<Arg>) -> App {
        for arg in args.into_iter() {
            self = self.arg(arg);
        }
        self
    }

    /// Adds a subcommand to the list of valid possibilties. Subcommands
    /// are effectively sub apps, because they can contain their own arguments
    /// and subcommands. They also function just like apps, in that they get their
    /// own auto generated help and version switches.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app = App::new("myprog")
    /// .subcommand(SubCommand::new("config")
    ///                .about("Controls configuration features")
    ///                .arg(Arg::new("config_file")
    ///                        .index(1)
    ///                        .help("Configuration file to use")))
    ///             // Additional subcommand configuration goes here, such as arguments...
    /// # .get_matches();
    /// ```
    pub fn subcommand(mut self, subcmd: App) -> App {
        if subcmd.name == "help" { self.needs_subcmd_help = false; }
        self.subcommands.insert(subcmd.name, Box::new(subcmd));
        self
    }

    /// Adds multiple subcommands to the list of valid possibilties
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app = App::new("myprog")
    /// .subcommands( vec![
    ///        SubCommand::new("config").about("Controls configuration functionality")
    ///                                 .arg(Arg::new("config_file").index(1)),
    ///        SubCommand::new("debug").about("Controls debug functionality")])
    /// # .get_matches();
    /// ```
    pub fn subcommands(mut self, subcmds: Vec<App>) -> App {
        for subcmd in subcmds.into_iter() {
            self = self.subcommand(subcmd);
        }
        self
    }


    fn exit(&self) {
        unsafe { libc::exit(0); }
    }

    fn report_error(&self, msg: String, help: bool, quit: bool) {
        println!("{}", msg);
        if help { self.print_usage(true); }
        if quit { env::set_exit_status(1); self.exit(); }
    }

    fn print_usage(&self, more_info: bool) {
        println!("USAGE:");
        if let Some(u) = self.usage_str {
            println!("\t{}",u);
        } else {
            let flags = ! self.flags.is_empty();
            let pos = ! self.positionals_idx.is_empty();
            let req_pos = self.positionals_idx.values().filter_map(|ref x| if x.required { Some(x.name) } else {None})
                                                       .fold(String::new(), |acc, ref name| acc + &format!("<{}> ", name)[..]);
            let req_opts = self.opts.values().filter(|ref x| x.required)
                                             .fold(String::new(), |acc, ref o| acc + &format!("{}{} ",if let Some(s) = o.short {
                                                                                                     format!("-{} ", s)
                                                                                                   } else {
                                                                                                       format!("--{}=",o.long.unwrap())
                                                                                                   },o.name));
            let opts = ! self.opts.is_empty();
            let subcmds = ! self.subcommands.is_empty();

            print!("\t{} {} {} {} {}", if let Some(ref name) = self.bin_name { &name[..] } else { self.name },
                if flags {"[FLAGS]"} else {""},
                if opts {
                    if req_opts.is_empty() { "[OPTIONS]" } else { &req_opts[..] } 
                } else { "" },
                if pos {
                    if req_pos.is_empty() { "[POSITIONAL]"} else { &req_pos[..] }
                } else {""},
                if subcmds {"[SUBCOMMANDS]"} else {""});
        }

        if more_info {
            println!("\nFor more information try --help");
        }
    }

    fn print_help(&self) {
        self.print_version(false);
        let flags = ! self.flags.is_empty();
        let pos = ! self.positionals_idx.is_empty();
        let opts = ! self.opts.is_empty();
        let subcmds = ! self.subcommands.is_empty();

        if let Some(author) = self.author {
            println!("{}", author);
        }
        if let Some(about) = self.about {
            println!("{}", about);
        }
        println!("");
        self.print_usage(false);
        if flags || opts || pos || subcmds {
            println!("");
        }
        if flags {
            println!("");
            println!("FLAGS:");
            for v in self.flags.values() {
                println!("\t{}{}\t{}",
                        if let Some(s) = v.short{format!("-{}",s)}else{format!("   ")},
                        if let Some(l) = v.long {format!(",--{}",l)}else {format!("   \t")},
                        if let Some(h) = v.help {h} else {"   "} );
            }
        }
        if opts {
            println!("");
            println!("OPTIONS:");
            for v in self.opts.values() {
                let mut needs_tab = false;
                println!("\t{}{}{}\t{}",
                        if let Some(ref s) = v.short{format!("-{} ",s)}else{format!("   ")},
                        if let Some(ref l) = v.long {format!(",--{}=",l)}else {needs_tab = true; format!(" ")},
                        format!("{}", v.name),
                        if let Some(ref h) = v.help {if needs_tab {format!("\t{}", *h)} else { format!("{}", *h) } } else {format!("   ")} );
            }
        }
        if pos {
            println!("");
            println!("POSITIONAL ARGUMENTS:");
            for v in self.positionals_idx.values() {
                println!("\t{}\t\t\t{}", v.name,
                        if let Some(h) = v.help {h} else {"   "} );
            }
        }
        if subcmds {
            println!("");
            println!("SUBCOMMANDS:");
            for sc in self.subcommands.values() {
                println!("\t{}\t\t{}", sc.name,
                    if let Some(a) = sc.about {a} else {"   "} );
            }
        }

        self.exit();
    }

    fn print_version(&self, quit: bool) {
        println!("{} {}", self.name, if let Some(v) = self.version {v} else {""} );
        if quit { self.exit(); }
    }

    fn check_for_help_and_version(&self, arg: char) {
        if arg == 'h' && self.needs_short_help {
            self.print_help();
        } else if arg == 'v' && self.needs_short_version {
            self.print_version(true);
        }
    }

    fn parse_long_arg(&mut self, matches: &mut ArgMatches ,full_arg: &String) -> Option<&'static str> {
        let mut arg = full_arg.trim_left_matches(|c| c == '-');
        let mut found = false;

        if arg == "help" && self.needs_long_help {
            self.print_help();
        } else if arg == "version" && self.needs_long_version {
            self.print_version(true);
        }

        let mut arg_val: Option<String> = None;

        if arg.contains("=") {
            let arg_vec: Vec<&str> = arg.split("=").collect();
            arg = arg_vec[0];
            arg_val = Some(arg_vec[1].to_string());
        } 

        for (k, v) in self.opts.iter() {
            if let Some(ref l) = v.long {
                if *l == arg {
                    if self.blacklist.contains(k) {
                        self.report_error(format!("The argument --{} is mutually exclusive with one or more other arguments", arg),
                            true, true);
                    }

                    matches.opts.insert(k, OptArg{
                        name: v.name,
                        short: v.short,
                        long: v.long, 
                        help: v.help,
                        required: v.required,
                        blacklist: None,
                        multiple: v.multiple,
                        requires: None,
                        values: if arg_val.is_some() { vec![arg_val.clone().unwrap()]} else {vec![]} 
                    });
                      
                    match arg_val {
                        None => { return Some(v.name); },
                        _ => { return None; }
                    } 
                }
            }
        } 

        for (k, v) in self.flags.iter() {
            if let Some(ref l) = v.long {
                if *l != arg { continue; }
                found = true;
                let mut multi = false;
                if let Some(ref mut f) = matches.flags.get_mut(k) {
                    f.occurrences = if f.multiple { f.occurrences + 1 } else { 1 };
                    multi = true;
                }  
                if ! multi { 
                    if self.blacklist.contains(k) {
                        self.report_error(format!("The argument --{} is mutually exclusive with one or more other arguments", arg),
                            true, true);
                    }
                    matches.flags.insert(k, FlagArg{
                        name: v.name,
                        short: v.short,
                        long: v.long,
                        help: v.help,
                        multiple: v.multiple,
                        occurrences: v.occurrences,
                        blacklist: None, 
                        requires: None
                    });
                    if self.required.contains(k) {
                        self.required.remove(k);
                    }
                    if let Some(ref bl) = v.blacklist {
                        if ! bl.is_empty() {
                            for name in bl.iter() {
                                self.blacklist.insert(name);
                            }
                        }
                    }
                }
                if let Some(ref reqs) = v.requires {
                    if ! reqs.is_empty() {
	                	// Add all required args which aren't already found in matches to the
	                	// final required list
	                    for n in reqs.iter().filter(|&a|
	                        ! matches.opts.contains_key(a) ||
	                        ! matches.flags.contains_key(a) ||
	                        ! matches.positionals.contains_key(a) ) {

	                    	self.required.insert(n);
	                    }
                    }
                }
                break;
            }
        }

        if ! found {
            self.report_error(
                format!("Argument --{} isn't valid", arg),
                true, true);
        }
        None
    }

    fn parse_short_arg(&mut self, matches: &mut ArgMatches ,full_arg: &String) -> Option<&'static str> {
        let arg = &full_arg[..].trim_left_matches(|c| c == '-');
        if arg.len() > 1 { 
            // Multiple flags using short i.e. -bgHlS
            for c in arg.chars() {
                self.check_for_help_and_version(c);
                if ! self.parse_single_short_flag(matches, c) { 
                    self.report_error(
                        format!("Argument -{} isn't valid",c),
                        true, true);
                }
            }
            return None;
        } 
        // Short flag or opt
        let arg_c = arg.chars().nth(0).unwrap();

        // Ensure the arg in question isn't a help or version flag
        self.check_for_help_and_version(arg_c);

        // Check for a matching flag, and return none if found
        if self.parse_single_short_flag(matches, arg_c) { return None; }
        
        // Check for matching short in options, and return the name
        // (only ones with shorts, of course)
        for v in self.opts.values().filter(|&v| v.short.is_some()) {
            if v.short.unwrap() == arg_c {
                return Some(v.name)
            }
        } 

        // Didn't match a flag or option, must be invalid
        self.report_error( format!("Argument -{} isn't valid",arg_c), true, true);

        unreachable!();
    }

    fn parse_single_short_flag(&mut self, matches: &mut ArgMatches, arg: char) -> bool {
        for (k, v) in self.flags.iter() {
            if let Some(s) = v.short {
                if s != arg { continue; }

                if !matches.flags.contains_key(k) {
                    if self.blacklist.contains(k) {
                        self.report_error(format!("The argument -{} is mutually exclusive with one or more other arguments", arg),
                            false, true);
                    }
                    matches.flags.insert(k, FlagArg{
                        name: v.name,
                        short: v.short,
                        long: v.long,
                        help: v.help,
                        multiple: v.multiple,
                        occurrences: v.occurrences,
                        blacklist: None, 
                        requires: None
                    });
                    if self.required.contains(k) {
                        self.required.remove(k);
                    }
                    if let Some(ref reqs) = v.requires {
                        if ! reqs.is_empty() {
                            for n in reqs.iter() {
                                if matches.opts.contains_key(n) { continue; }
                                if matches.flags.contains_key(n) { continue; }
                                if matches.positionals.contains_key(n) { continue; }
                                self.required.insert(n);
                            }
                        }
                    }
                    if let Some(ref bl) = v.blacklist {
                        if ! bl.is_empty() {
                            for name in bl.iter() {
                                self.blacklist.insert(name);
                            }
                        }
                    }
                } else if matches.flags.get(k).unwrap().multiple { 
                    matches.flags.get_mut(k).unwrap().occurrences += 1
                }

                return true;
            }
        }
        false
    }

    fn validate_blacklist(&self, matches: &ArgMatches) {
        if ! self.blacklist.is_empty() {
            for name in self.blacklist.iter() {
                for (k, v) in matches.flags.iter() {
                    if k == name {
                        self.report_error(format!("The argument {} is mutually exclusive with one or more other arguments",
                            if let Some(s) = v.short {
                                format!("-{}", s)
                            } else if let Some(l) = v.long {
                                format!("--{}", l)
                            } else {
                                format!("\"{}\"", v.name)
                            }),
                            true, true);
                    }
                }
                for (k, v) in matches.opts.iter() {
                    if k == name {
                        self.report_error(format!("The argument {} is mutually exclusive with one or more other arguments",
                            if let Some(s) = v.short {
                                format!("-{}", s)
                            } else if let Some(l) = v.long {
                                format!("--{}", l)
                            } else {
                                format!("\"{}\"", v.name)
                            }),
                            true, true);
                    }
                }
                for (k, v) in matches.positionals.iter() {
                    if k == name {
                        self.report_error(format!("The argument \"{}\" is mutually exclusive with one or more other arguments",v.name),
                            false, true);
                    }
                }
            }
        }
    }

    fn create_help_and_version(&mut self) {
        if self.needs_long_help {
            self.flags.insert("clap_help", FlagArg{
                name: "clap_help",
                short: if self.needs_short_help { Some('h') } else { None },
                long: Some("help"),
                help: Some("Prints this message"),
                blacklist: None,
                multiple: false,
                requires: None,
                occurrences: 1
            });
        }
        if self.needs_long_version {
            self.flags.insert("clap_version", FlagArg{
                name: "clap_version",
                short: if self.needs_short_help { Some('v') } else { None },
                long: Some("version"),
                help: Some("Prints version information"),
                blacklist: None,
                multiple: false,
                requires: None,
                occurrences: 1
            });
        }
        if self.needs_subcmd_help && ! self.subcommands.is_empty() {
            self.subcommands.insert("help", Box::new(App::new("help").about("Prints this message")));
        }
    }

    fn get_matches_from(&mut self, matches: &mut ArgMatches, it: &mut IntoIter<String>) {
        self.create_help_and_version();

        let mut pos_only = false;
        let mut subcmd_name: Option<&'static str> = None;
        let mut needs_val_of: Option<&'static str> = None; 
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_slice = &arg[..];
            let mut skip = false;
            if ! pos_only {
                if let Some(nvo) = needs_val_of {
                    if let Some(ref opt) = self.opts.get(nvo) {
                        if self.blacklist.contains(opt.name) {
                            self.report_error(
                                format!("The argument {} is mutually exclusive with one or more other arguments", 
                                if let Some(long) = opt.long {
                                    format!("--{}",long)
                                }else{
                                    format!("-{}",opt.short.unwrap())
                                }),true, true);
                        }
                        let mut done = false;
                        if opt.multiple {
                            if let Some(ref mut o) = matches.opts.get_mut(opt.name) {
                                done = true;
                                o.values.push(arg.clone());
                            } 
                        } 
                        if ! done {
                            matches.opts.insert(nvo, OptArg{
                                name: opt.name,
                                short: opt.short,
                                long: opt.long, 
                                help: opt.help,
                                requires: None,
                                blacklist: None,
                                multiple: opt.multiple,
                                required: opt.required,
                                values: vec![arg.clone()] 
                            });
                        }
                        if let Some(ref bl) = opt.blacklist {
                            if ! bl.is_empty() {
                                for name in bl.iter() {
                                    self.blacklist.insert(name);
                                }
                            }
                        }
                        if self.required.contains(opt.name) {
                            self.required.remove(opt.name);
                        }
                        if let Some(ref reqs) = opt.requires {
                            if ! reqs.is_empty() {
	                        	// Add all required args which aren't already found in matches to the
	                        	// final required list
	                            for n in reqs.iter().filter(|&a|
	                                ! matches.opts.contains_key(a) ||
	                                ! matches.flags.contains_key(a) ||
	                                ! matches.positionals.contains_key(a) ) {

	                            	self.required.insert(n);
	                            }
	                        }
                        }
                        skip = true;
                    }
                }
            }
            if skip {
                needs_val_of = None;
                continue;
            }
            if arg_slice.starts_with("--") && ! pos_only {
                if arg_slice.len() == 2 {
                    pos_only = true;
                    continue;
                }
                // Single flag, or option long version
                needs_val_of = self.parse_long_arg(matches, &arg);
            } else if arg_slice.starts_with("-") && arg_slice.len() != 1 && ! pos_only {
                needs_val_of = self.parse_short_arg(matches, &arg);
            } else {
                // Positional or Subcommand
                if let Some(sca) = self.subcommands.get(arg_slice) {
                    if sca.name == "help" {
                        self.print_help();
                    }
                    subcmd_name = Some(sca.name);
                    break;
                }

                if self.positionals_idx.is_empty() {
                    self.report_error(
                        format!("Found positional argument {}, but {} doesn't accept any", arg, self.name),
                        true, true);
                }
                if let Some(ref p) = self.positionals_idx.get(&pos_counter) {
                    if self.blacklist.contains(p.name) {
                        self.report_error(format!("The argument \"{}\" is mutually exclusive with one or more other arguments", arg),
                            true, true);
                    }
                    matches.positionals.insert(p.name, PosArg{
                        name: p.name,
                        help: p.help,
                        required: p.required,
                        blacklist: None,
                        requires: None,
                        value: Some(arg.clone()),
                        index: pos_counter
                    });
                    if let Some(ref bl) = p.blacklist {
                        if ! bl.is_empty() {
                            for name in bl.iter() {
                                self.blacklist.insert(name);
                            }
                        }
                    }
                    if self.required.contains(p.name) {
                        self.required.remove(p.name);
                    }
                    if let Some(ref reqs) = p.requires {
                        if ! reqs.is_empty() {
                        	// Add all required args which aren't already found in matches to the
                        	// final required list
                            for n in reqs.iter().filter(|&a|
                                ! matches.opts.contains_key(a) ||
                                ! matches.flags.contains_key(a) ||
                                ! matches.positionals.contains_key(a) ) {

                            	self.required.insert(n);
                            }
                        }
                    }
                    pos_counter += 1;
                } else {
                    self.report_error(format!("Positional argument \"{}\" was found, but {} wasn't expecting any", arg, self.name), true, true);
                }
            }
        }

        match needs_val_of {
            Some(ref a) => {
                self.report_error(
                    format!("Argument \"{}\" requires a value but none was supplied", a),
                    true, true);
            }
            _ => {}
        }
        if ! self.required.is_empty() {
            self.report_error("One or more required arguments were not supplied".to_string(),
                    true, true);
        }

        self.validate_blacklist(&matches);

        if let Some(sc_name) = subcmd_name {
            if let Some(ref mut sc) = self.subcommands.get_mut(sc_name) {
                let mut new_matches = ArgMatches::new(sc_name);
                sc.get_matches_from(&mut new_matches, it);
                matches.subcommand = Some((sc_name, Box::new(SubCommand{
                    name: sc_name,
                    matches: new_matches})));
            }
        }    
    }

    pub fn get_matches(mut self) -> ArgMatches {
        let mut matches = ArgMatches::new(self.name);

        let args = env::args().collect::<Vec<_>>();    
        let mut it = args.into_iter();
        if let Some(name) = it.next() {
            let p = Path::new(&name[..]);
            if let Some(f) = p.file_name() {
                match f.to_os_string().into_string() {
                    Ok(s) => self.bin_name = Some(s),
                    Err(_) => {}
                }
            }
        }
        self.get_matches_from(&mut matches, &mut it );

        matches
    }
}
