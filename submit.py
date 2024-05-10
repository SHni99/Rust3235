# Script to clean up the workspace and compress this folder into a .zip file
import os
import sys
import shutil
import json

CUR_DIR = os.path.dirname(os.path.realpath(__file__))

# ------- Clean up workspace -------
# Delete `target` folder inside workspace: `workspace/short/target` and `workspace/long/target`
def clean_workspace(flag):
  workspace_dir = os.path.join(CUR_DIR, "workspace")
  for root, dirs, files in os.walk(workspace_dir):
    for d in dirs:
      if d == "target":
        if flag:
          print("Deleting", os.path.join(root, d))
          shutil.rmtree(os.path.join(root, d))
        else:
          print("To be deleted", os.path.join(root, d))

# ------- Compress workspace -------
# First check if `report.pdf` exists in the workspace. If not, exit
# Then read the student ID in `../selected.json` using the key `student_id`
# Compress the workspace into `student_id_proj.zip` file
def compress_workspace(flag):
  if not os.path.exists("./workspace/report.pdf"):
    print("Error: report.pdf not found in workspace. Please put the report with name `report.pdf` in the workspace folder.")
    return

  with open("./selected.json", "r") as f:
    student_id = json.load(f)["student_id"]


  if flag:  
    print("Compressing workspace into", student_id + "_proj.zip")
    shutil.make_archive(student_id + "_proj", "zip", "workspace")
  else:
    print("To be compressed into", student_id + "_proj.zip")

if __name__ == "__main__":
  # helper information
  if len(sys.argv) == 2 and sys.argv[1] == "-h":
    print("Usage: python3 submit.py [yes]")
    print("Use `yes` to confirm the cleanup and compression.")
    print("Without `yes`, only printing the folders to be deleted. Don't delete them")
    exit()


  # check if executing this script in the correct directory, i.e., the parent folder of `workspace`
  # if not, exit
  if not os.path.exists("./workspace"):
    print("Error: workspace not found")
    exit()
  
  # if with arg, save it to `flag`
  flag = None
  if len(sys.argv) > 1:
      flag = sys.argv[1]
      if flag != "yes":
        print("Error: Invalid flag. Please use `yes` to confirm the cleanup and compression.")
        exit()
      else:
        flag = True
        print("Cleaning up workspace...")
  else:
    print("[NOTE] Without flag, only printing the folders to be deleted. Use `yes` to confirm the cleanup and compression, e.g., `python3 submit.py yes`.")

  clean_workspace(flag)
  compress_workspace(flag)