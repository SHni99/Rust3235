# Script to initialize workspaces in this project
import sys
import hashlib
import os
import shutil
import json
import subprocess

CUR_DIR = os.path.dirname(os.path.realpath(__file__))
SOURCE_DIR = os.path.join(CUR_DIR, "source")
SOURCE_SHORT_DIR = os.path.join(SOURCE_DIR, "short")
SOURCE_LONG_DIR = os.path.join(SOURCE_DIR, "long")
SOURCE_LONG_FORTRANS_DIR = os.path.join(SOURCE_LONG_DIR, "__for_trans")

WORKSPACE_DIR = os.path.join(CUR_DIR, "workspace")
WORKSPACE_SHORT_DIR = os.path.join(WORKSPACE_DIR, "short")
WORKSPACE_LONG_DIR = os.path.join(WORKSPACE_DIR, "long")


main_rs = """#[path="../bindings/bindings.rs"]
mod bindings;
use bindings::*;

// Please translate these functions to their Rust version with `_rs` suffix, e.g., `awk_main` -> `awk_main_rs`
// selected
// Translate `main` function first, then the above functions one by one in order according to the call graph

// If a function is translated, please call its Rust version in Rust side
// e.g., when `awk_main` is translated to `awk_main_rs`, call `awk_main_rs` instead of `awk_main` in `main` function

// fn awk_main_rs(...) {
//    ...
// }

// ...

fn main() {
    println!("Translate `main` function first!");
    // parse arguments to C function awk_main
    // ...
    // awk_main(...)
    // ...
}
"""

# ------- Pick the program and function to work on -------

short_programs_list = [
  "bsd-csplit",
  "bsd-expr", 
  "bsd-fmt", 
  "bsd-join", 
  "bsd-printf", 
  "bsd-test", 
  "b-shoco", 
  "b-urlparser"
]

# awk_main 113 LoC

long_program_func_list = [
  [
    "awk_main", # 113
    "awk_getline", # 100
    "setvar_i", # 6
    "setvar_s", # 1
  ],
  [
    "awk_main", # 113
    "handle_special", # 60
    "getvar_i", # 24
    "getvar_s", # 10
    "setvar_p", # 4
    "istrue", # 3
  ],
    # "mk_splitter", # 20
  [
    "awk_main", # 113
    "parse_program", # 70
    "clear_array", # 15
    "chain_node", # 15
  ],
  [
    "awk_main", # 113
    "next_input_file", # 30
    "getvar_i", # 24
    "getvar_s", # 10
    "setvar_s", # 1
    "is_assignment", # 20
    "iamarray", # 10
    "incvar", # 1
    "nextword", # 4
    "setvar_p", # 4
    "setvar_i", # 6
    "unescape_string_in_place", # 5
    "llist_pop", # 10
  ]
]

def random_pick_item(base_string, input_string, items_list, mod_num):
    # Combine the base string with the input string
    combined_string = base_string + input_string
    # Create a hash of the combined string
    hashed = hashlib.sha256(combined_string.encode()).hexdigest()
    # Convert the hash to an integer
    seed = int(hashed, 16) % mod_num
    return items_list[seed]

def pick_per_stu(student_id):
    res = {}
    selected_program = random_pick_item("CS3235sp", student_id, short_programs_list, len(short_programs_list))
    selected_function = random_pick_item("cs3235_long", student_id, long_program_func_list, len(long_program_func_list))
    res["part a"] = selected_program
    res["part b"] = selected_function
    res['student_id'] = student_id
    print("Selected program in Part A: ", str(selected_program))
    print("Selected function in Part B: ", str(selected_function))

    # Save the result to a file `selected.txt`
    print("Saving the selected program and functions to selected.json")
    with open(os.path.join(CUR_DIR, "selected.json"), "w") as f:
      json.dump(res, f, indent=2)
    return res


# ------- Initialize the workspace -------

# For Part. A, copy the selected program folder to the workspace
def init_part_a(selected_res):
    program_name = selected_res["part a"]
    # create a folder WORKSPACE_SHORT_DIR
    os.makedirs(WORKSPACE_SHORT_DIR, exist_ok=True)

    prog_src_folder = os.path.join(SOURCE_SHORT_DIR, program_name)
    prog_dest_folder = os.path.join(WORKSPACE_SHORT_DIR, "orig_c")
    # copy prog_src_folder to prog_dest_folder using shutil
    print("Copying {} to {}".format(prog_src_folder, prog_dest_folder))
    shutil.copytree(prog_src_folder, prog_dest_folder)
    # move the files in `orig_c/__for_trans` folder to `WORKSPACE_SHORT_DIR`
    prog_dest_fortrans_folder = os.path.join(prog_dest_folder, "__for_trans")
    for file in os.listdir(prog_dest_fortrans_folder):
      src_file = os.path.join(prog_dest_fortrans_folder, file)
      dest_file = os.path.join(WORKSPACE_SHORT_DIR, file)
      shutil.move(src_file, dest_file)
    os.rmdir(prog_dest_fortrans_folder)
    # copy SOURCE_SHORT_DIR/README.md and SOURCE_SHORT_DIR/trans_prompts.py to WORKSPACE_SHORT_DIR
    shutil.copy(os.path.join(SOURCE_SHORT_DIR, "README.md"), WORKSPACE_SHORT_DIR)
    shutil.copy(os.path.join(SOURCE_SHORT_DIR, "trans_prompts.py"), WORKSPACE_SHORT_DIR)

def init_part_b(selected_res):
    function_name = selected_res["part b"]
    # execute the cmd `cargo init --bin awk` in the workspace
    try:
      subprocess.run(["cargo", "init", "--bin", "awk_rs"], cwd=WORKSPACE_DIR, check=True)
      os.rename(os.path.join(WORKSPACE_DIR, "awk_rs"), WORKSPACE_LONG_DIR)
    except subprocess.CalledProcessError as e:
      print("Error: ", e)
      return

    func_src_folder = os.path.join(SOURCE_LONG_DIR, "awk")
    func_dest_folder = os.path.join(WORKSPACE_LONG_DIR, "orig_c")
    # copy func_src_folder to func_dest_folder using shutil
    print("Copying {} to {}".format(func_src_folder, func_dest_folder))
    shutil.copytree(func_src_folder, func_dest_folder)

    # copy README.md to WORKSPACE_LONG_DIR
    shutil.copy(os.path.join(SOURCE_LONG_DIR, "README.md"), WORKSPACE_LONG_DIR)

    # copy files and dirs in SOURCE_LONG_FORTRANS_DIR to WORKSPACE_LONG_DIR
    print("Copying files and dirs in {} to {}".format(SOURCE_LONG_FORTRANS_DIR, WORKSPACE_LONG_DIR))
    for file in os.listdir(SOURCE_LONG_FORTRANS_DIR):
      if os.path.isfile(os.path.join(SOURCE_LONG_FORTRANS_DIR, file)):
        src_file = os.path.join(SOURCE_LONG_FORTRANS_DIR, file)
        dest_file = os.path.join(WORKSPACE_LONG_DIR, file)
        shutil.copy(src_file, dest_file)
    for subdir in os.listdir(SOURCE_LONG_FORTRANS_DIR):
      if os.path.isfile(os.path.join(SOURCE_LONG_FORTRANS_DIR, subdir)):
        continue
      src_subdir = os.path.join(SOURCE_LONG_FORTRANS_DIR, subdir)
      dest_subdir = os.path.join(WORKSPACE_LONG_DIR, subdir)
      shutil.copytree(src_subdir, dest_subdir)

    # add functions to be translated into comments of `src/main.rs`
    with open(os.path.join(WORKSPACE_LONG_DIR, "src", "main.rs"), "w") as f:
      f.write(main_rs.replace("selected", ", ".join(function_name)))

def init_workspace(selected_res):
    if os.path.exists(WORKSPACE_DIR):
        print("Failed to Initialize: Workspace already exists.")
        exit(1)
    os.makedirs(WORKSPACE_DIR, exist_ok=True)
    init_part_a(selected_res)
    init_part_b(selected_res)


if __name__ == "__main__":
  # Takes the student ID as input, e.g., "A123456N"
  # Generate a random number from it
  # Sample one from the short_programs_list
  if len(sys.argv) != 2:
    print("Usage: python3 initialize.py <student_id>. E.g., python3 initialize.py A123456N")
    sys.exit(1)
  
  student_id = sys.argv[1].strip()
  # check if the student_id is capitalized
  if student_id != student_id.upper():
    print("Student ID should be capitalized. E.g., A123456N")
    sys.exit(1)

  selected_res = pick_per_stu(student_id)
  init_workspace(selected_res)
  print("Workspace initialized done.")
  