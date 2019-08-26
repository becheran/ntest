import toml
from pathlib import Path
import os 

class Version:
    x = 5

    def increase_major(version):



    def increase_minor(version):

    def increase_patch(version):

def main():
    file_dir = os.path.dirname(os.path.realpath(__file__))
    ntest_toml_path = os.path.join(file_dir, 'ntest', 'Cargo.toml')
    with open(ntest_toml_path, 'r') as toml_file:
        data = toml.loads(toml_file.read())
        current_version = data['package']['version']


    ntest_test_cases_toml_path = os.path.join(file_dir, 'ntest', 'Cargo.toml')

def parse_version_string(string):
    version.split('.')






if __name__ == "__main__":
    main()