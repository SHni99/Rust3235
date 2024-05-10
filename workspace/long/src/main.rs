#[path="../bindings/bindings.rs"]
mod bindings;
use bindings::*;
use std::env;
use libc::{strchr, c_char, c_int, c_uint, c_void, c_double, free, setlocale, LC_NUMERIC};
use std::ffi::{CStr, CString};
use std::process;

extern "C" {
    #[cfg(target_os = "linux")]
    pub static mut environ: *mut *mut c_char;
}


fn chain_node_rs(mut info: u32) -> *mut node {
	//let mut n: *mut node = std::ptr::null_mut();
    unsafe {
        if (*(*ptr_to_globals.offset(-1)).seq).first.is_null() {
            (*(*ptr_to_globals.offset(-1)).seq).last = new_node(0);
            (*(*ptr_to_globals.offset(-1)).seq).first = (*(*ptr_to_globals.offset(-1)).seq).last;
        }

        if ((*(*ptr_to_globals.offset(-1)).seq).programname != (*ptr_to_globals.offset(-1)).g_progname) {
            (*(*ptr_to_globals.offset(-1)).seq).programname = (*ptr_to_globals.offset(-1)).g_progname;
            let n = chain_node_rs(OC_NEWSOURCE as c_int as u32);
            
            (*n).l.new_progname = xstrdup((*ptr_to_globals.offset(-1)).g_progname);
            
        }
    }
    
    let last = unsafe {(*(*ptr_to_globals.offset(-1)).seq).last};
    
    unsafe {
        (*last).info = info;
        (*last).a.n = new_node(OC_DONE as c_int as u32);
        (*(*ptr_to_globals.offset(-1)).seq).last = (*last).a.n;
    }
        return last;
}

fn clear_array_rs(mut array: *mut xhash) {
    unsafe {
        let mut i: c_uint = 0;
        let mut hi: *mut hash_item = std::ptr::null_mut();
        let mut thi: *mut hash_item = std::ptr::null_mut();
        i = 0 as c_uint;
        while i < (*array).csize {
        hi = *(*array).items.offset(i as isize);
        while !hi.is_null() {
            thi = hi;
            hi = (*hi).next;
            free((*thi).data.v.string as *mut c_void);
            free(thi as *mut c_void);
        }
        *(*array).items.offset(i as isize) = std::ptr::null_mut();
        i += 1;
        }
        (*array).nel = 0 as c_uint;
        (*array).glen = (*array).nel;
    }
}

fn parse_program_rs(mut p: *mut c_char) {

    #[derive(Debug)]
    enum TokenClass {
        TC_OPTERM,
        TC_BEGIN,
        TC_END,
        TC_FUNCDECL,
        TC_OPSEQ,
        TC_GRPSTART,
        TC_EOF,
    }
    unsafe {
        let mut tclass: u32 = 0;
        let mut cn: *mut node = std::ptr::null_mut();
        let mut f: *mut func = std::ptr::null_mut();
        let mut v: *mut var = std::ptr::null_mut();

        (*ptr_to_globals.offset(-1)).g_pos = p;
        (*(ptr_to_globals as *mut globals2)).t_lineno = 1i32;

        while {
            tclass = next_token(TC_EOF | TC_OPSEQ | TC_GRPSTART | TC_OPTERM | TC_BEGIN | TC_END | TC_FUNCDECL) as u32;
            tclass != TC_EOF
        } {
                
            if tclass & TC_OPTERM !=0 {
                continue;
            }

            (*ptr_to_globals.offset(-1)).seq = &mut (*ptr_to_globals.offset(-1)).mainseq;

            if tclass & TC_BEGIN != 0 {
                (*ptr_to_globals.offset(-1)).seq = &mut (*ptr_to_globals.offset(-1)).beginseq;
                chain_group();
            } else if tclass & TC_END != 0 {
                (*ptr_to_globals.offset(-1)).seq = &mut (*ptr_to_globals.offset(-1)).endseq;
                chain_group();
            } else if tclass & TC_FUNCDECL != 0 {
                next_token(TC_FUNCTION);
                let ref mut fresh24 = (*ptr_to_globals.offset(-1)).g_pos;
                *fresh24 = (*fresh24).add(1);
                f = hash_find(
                    (*ptr_to_globals.offset(-1)).fnhash,
                    (*(ptr_to_globals as *mut globals2)).t_string,
                ) as *mut func;  //func_s??
                (*f).body.first = std::ptr::null_mut();
                (*f).nargs = 0 as c_uint;
            

                while (next_token(TC_VARIABLE | TC_SEQTERM | TC_COMMA) as u32) != 0 {
                    
                    if (*f).nargs == 0 as c_uint && (*(ptr_to_globals as *mut globals2)).t_tclass ==  TC_SEQTERM as c_uint {
                        break;
                    }
                    /* TC_SEQSTART/TC_COMMA must be followed by TC_VARIABLE */
                    if (*(ptr_to_globals as *mut globals2)).t_tclass != TC_VARIABLE as c_uint {
                        syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
                    }
                    v = hash_find(
                        (*ptr_to_globals.offset(-1)).ahash,
                        (*(ptr_to_globals as *mut globals2)).t_string,
                    ) as *mut var; //var_s?
                    let fresh25 = (*f).nargs;
                    (*f).nargs += 1;
                    (*v).x.aidx = fresh25 as c_int;
                    /* Arg followed either by end of arg list or 1 comma */
                    if (next_token(TC_COMMA | TC_SEQTERM) & TC_SEQTERM) as c_uint != 0 {
                        break;
                    }
                    if (*(ptr_to_globals as *mut globals2)).t_tclass != TC_COMMA as c_uint {
                        syntax_error(EMSG_UNEXP_TOKEN.as_ptr());
                    }
                }
                (*ptr_to_globals.offset(-1)).seq = &mut (*f).body;
                chain_group();
                clear_array_rs((*ptr_to_globals.offset(-1)).ahash);

            } else if tclass & TC_OPSEQ != 0 {
                rollback_token();
                cn = chain_node_rs(OC_TEST as c_int as u32);
                (*cn).l.n = parse_expr(TC_OPTERM | TC_EOF | TC_GRPSTART);
                if ((*(ptr_to_globals as *mut globals2)).t_tclass & TC_GRPSTART as c_uint) != 0 {
                rollback_token();
                chain_group();
                } else {
                chain_node_rs(OC_PRINT as c_int as u32);
                }
                (*cn).r.n = (*ptr_to_globals.offset(-1)).mainseq.last
            } else {
                /* if (tclass & TC_GRPSTART) */
                rollback_token();
                chain_group();
            }
        }
    }   
}

fn awk_main_rs(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {

    let mut tv: var = var {
        type_: 0,
        number: 0.,
        string: std::ptr::null_mut::<libc::c_char>(),
        x: var_s__bindgen_ty_1 { aidx: 0 },
    }; 
    
    unsafe {
        let mut opt: c_uint = 0;
        let mut opt_F: *mut c_char = std::ptr::null_mut::<libc::c_char>();
        let mut list_v: *mut llist_t = std::ptr::null_mut();
        let mut list_f: *mut llist_t = std::ptr::null_mut();
        let mut list_e: *mut llist_t = std::ptr::null_mut();
        let mut i: c_int = 0;
        let mut j: c_int = 0;
        let mut v: *mut var = std::ptr::null_mut();
        let mut envp: *mut *mut c_char = std::ptr::null_mut();
        //let mut environ: *mut *mut libc::c_char;

        

        let mut vnames: *mut c_char = vNames.as_ptr() as *mut c_char;
        let mut vvalues: *mut c_char = vValues.as_ptr() as *mut c_char;

        init_g();

        if ENABLE_LOCALE_SUPPORT == 0 {
            setlocale(LC_NUMERIC, "C\0".as_ptr() as *const c_char);
        }

        (*(ptr_to_globals as *mut globals2)).evaluate__seed = 1i32 as c_uint;

        zero_out_var(&mut tv);

        let ref mut fresh9 = (*ptr_to_globals.offset(-1)).g_buf;
        *fresh9 = xmalloc((240 + 1).try_into().unwrap()) as *mut c_char;

        
        //Hash tables
        let ref mut vfresh = (*ptr_to_globals.offset(-1)).vhash;
        *vfresh = hash_init();
        let ref mut afresh = (*ptr_to_globals.offset(-1)).ahash;
        *afresh = hash_init();
        let ref mut fdfresh = (*ptr_to_globals.offset(-1)).fdhash;
        *fdfresh = hash_init();
        let ref mut fnfresh = (*ptr_to_globals.offset(-1)).fnhash;
        *fnfresh = hash_init();

        //println!("ok");
        

        i = 0;
        while *vnames != 0 {
            //println!("The value of i is: {}", i);
            v = hash_find((*ptr_to_globals.offset(-1)).vhash, nextword(&mut vnames)) as *mut var; 
            let ref mut fresh10 = (*(ptr_to_globals as *mut globals2)).intvar[i as usize];
            *fresh10 = v;  
            
            if *vvalues as c_int != '\u{FF}' as i32 {
                setvar_s(v, nextword(&mut vvalues));
            } else {
                
                setvar_i(v, 0 as c_double);
            }
            // Special variable handling
            if *vnames as c_int == '*' as i32 {
                (*v).type_ |= VF_SPECIAL;
                vnames = vnames.add(1);
            }   
            i += 1;
        }

        let fs_index = FS as usize;  
        handle_special((*(ptr_to_globals as *mut globals2)).intvar[fs_index]);
        
        let rs_index = RS as usize; 
        handle_special((*(ptr_to_globals as *mut globals2)).intvar[rs_index]);


        let stdin_cstr = CString::new("/dev/stdin").expect("CString::new failed");
        let ref mut sin = (*(hash_find((*ptr_to_globals.offset(-1)).fdhash, stdin_cstr.as_ptr()) as *mut rstream)).F;
        *sin = stdin;
        let stdout_cstr = CString::new("/dev/stdout").expect("CString::new failed");
        let ref mut sout = (*(hash_find((*ptr_to_globals.offset(-1)).fdhash, stdout_cstr.as_ptr()) as *mut rstream)).F;
        *sout = stdout;
        let stderr_cstr = CString::new("/dev/stderr").expect("CString::new failed");
        let ref mut serr = (*(hash_find((*ptr_to_globals.offset(-1)).fdhash, stderr_cstr.as_ptr()) as *mut rstream)).F;
        *serr = stderr;

        if environ.is_null() {
            envp = environ;
            while !(*envp).is_null() {
                let mut entry = *envp;
                let mut s1 = strchr(entry, '=' as i32);
                if !s1.is_null() {
                    *s1 = '\u{0}' as i32 as c_char;
                    setvar_u(hash_find(iamarray((*(ptr_to_globals as *mut globals2)).intvar[ENVIRON as usize]), entry) as *mut var, s1.add(1));
                    *s1 = '=' as i32 as c_char;
                }
                envp = envp.add(1);   
            }
        }
        
        opt = getopt32(
            argv,
            OPTSTR_AWK as *const u8 as *const c_char,
            &mut opt_F as *mut *mut c_char,
            &mut list_v as *mut *mut llist_t,
            &mut list_f as *mut *mut llist_t,
            if cfg!(feature = "ENABLE_FEATURE_AWK_GNU_EXTENSIONS") {
                &mut list_e
            } else {
                std::ptr::null_mut()
            }
        );
        
        argv = argv.add(getoptind().try_into().unwrap());

        if (opt & OPT_W != 0) {
            bb_simple_error_msg("warning: option -W is ignored".as_ptr() as *const c_char);
        }

        if (opt & OPT_F as c_int as c_uint != 0) {
            unescape_string_in_place(opt_F);
            setvar_s((*(ptr_to_globals as *mut globals2)).intvar[FS as usize], opt_F);
        }

        while !list_v.is_null() {
            let expr = llist_pop(&mut list_v) as *const c_char;
        
            // Call `is_assignment` and check if the return value is non-zero (which means true in C context)
            if is_assignment(expr) == 0 {
                bb_show_usage();
            }
        }

        
        while !list_f.is_null() {
            let mut s: *mut c_char = std::ptr::null_mut();
            let filename = llist_pop(&mut list_f) as *const c_char;
            let from_file = xfopen_stdin(filename);
            if from_file.is_null() {
                continue; 
            }

            j = 1;
            i = j;
            while j > 0 {
                s = xrealloc(s as *mut c_void, ((i as usize) + 4096)) as *mut c_char;
                if s.is_null() {
                    fclose(from_file);  // Ensure to close the file if realloc fails
                    break;
                }
                j = fread(s.add(i as usize) as *mut c_void, 1, 4094, from_file) as c_int;
                i += j;
            }

            *s.add(i as usize) = 0;  // Null-terminate the string
            fclose(from_file);
            parse_program_rs(s.add(1));  // Parse the program, skipping the first byte
            free(s as *mut c_void);
        }
        

        #[cfg(feature = "ENABLE_FEATURE_AWK_GNU_EXTENSIONS")]
        while !list_e.is_null() {
            parse_program_rs(llist_pop(&mut list_e));
        }

        if opt & (OPT_f | OPT_e) == 0 {
            if (*argv).is_null() {
                bb_show_usage();
            }
            let fresh1 = argv;
            argv = argv.add(1);
            parse_program_rs(*fresh1);
        }

        setari_u((*(ptr_to_globals as *mut globals2)).intvar[ARGV as usize], 0, CString::new("awk").unwrap().as_ptr());
        i = 0;
        while !(*argv).is_null() {
            i += 1;
            let fresh2 = argv;
            argv = argv.add(1);
            setari_u((*(ptr_to_globals as *mut globals2)).intvar[ARGV as usize], i, *fresh2);
        }

        setvar_i((*(ptr_to_globals as *mut globals2)).intvar[ARGC as usize], (i + 1) as c_double);

        evaluate((*ptr_to_globals.offset(-1)).beginseq.first, &mut tv);
        if (*ptr_to_globals.offset(-1)).mainseq.first.is_null() && (*ptr_to_globals.offset(-1)).endseq.first.is_null() {
            awk_exit(0);
        }

        if (*ptr_to_globals.offset(-1)).iF.is_null() {
            let ref mut fresh3 = (*ptr_to_globals.offset(-1)).iF;
            *fresh3 = next_input_file();
        }

        while !(*ptr_to_globals.offset(-1)).iF.is_null() {
            (*ptr_to_globals.offset(-1)).nextfile = 0_i8;
            setvar_i((*(ptr_to_globals as *mut globals2)).intvar[FNR as usize], 0 as c_double);

            while awk_getline((*ptr_to_globals.offset(-1)).iF, (*(ptr_to_globals as *mut globals2)).intvar[F0 as usize]) > 0 {
                (*ptr_to_globals.offset(-1)).nextrec = 0_i8;
                incvar((*(ptr_to_globals as *mut globals2)).intvar[NR as usize]);
                incvar((*(ptr_to_globals as *mut globals2)).intvar[FNR as usize]);
                evaluate((*ptr_to_globals.offset(-1)).mainseq.first, &mut tv);

                if (*ptr_to_globals.offset(-1)).nextfile != 0 {
                    break;
                }
            }

            if i < 0 {
                syntax_error(strerror((ERRNO as usize).try_into().unwrap()));
            }

            let ref mut fresh4 = (*ptr_to_globals.offset(-1)).iF;
            *fresh4 = next_input_file();
        }
    
    awk_exit(0);
    }
    0
}

// fn validate_args(argc: c_int, argv: *mut *mut c_char) -> Result<(), &'static str> {

//     if argc == 0 {
//         // No arguments, return Ok
//         return Ok(());
//     }

//     if argv.is_null() {
//         return Err("argv cannot be null");
//     }

//     for i in 0..argc {
//         let arg_ptr = unsafe { *argv.offset(i as isize) };
//         if arg_ptr.is_null() {
//             return Err("One of the arguments is null");
//         }

//         // Ensuring the argument is valid UTF-8 could also be part of the validation
//         let c_str = unsafe { CStr::from_ptr(arg_ptr) };
//         if c_str.to_str().is_err() {
//             return Err("Invalid UTF-8 sequence found in arguments");
//         }
//     }

//     Ok(())
// }

fn main() {
    let args: Vec<String> = std::env::args().collect(); // Collect command line arguments
    let cstrings: Vec<CString> = args.iter()
                                     .map(|arg| CString::new(arg.as_str()).unwrap())
                                     .collect(); // Convert to Vec<CString>
    let mut argv: Vec<*mut c_char> = cstrings.iter()
                                             .map(|cstring| cstring.as_ptr() as *mut c_char)
                                             .collect(); // Convert to Vec<*mut c_char>
    argv.push(std::ptr::null_mut()); // Add a null pointer at the end for C compatibility

    let argc = argv.len() as c_int - 1; // Calculate the number of arguments, excluding the null terminator
    let argv_ptr = argv.as_mut_ptr(); // Get a mutable pointer to the vector's buffer

    // Call the C function
    let ret = unsafe { awk_main(argc, argv_ptr) };
    std::process::exit(ret);
}

