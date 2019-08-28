import toml
from pathlib import Path
import os
from enum import Enum
import argparse
import subprocess

FILE_DIR = os.path.dirname(os.path.realpath(__file__))


class Version:

    def __init__(self, version: str):
        splited = version.split('.')
        if len(splited) != 3:
            raise Exception(
                'Unkown version string {} Must be of format <major>.<minor>.<patch>.'.format(version))
        self._major = int(splited[0])
        self._minor = int(splited[1])
        self._patch = int(splited[2])

    def __repr__(self):
        return '{}.{}.{}'.format(
            self._major,
            self._minor,
            self._patch,
        )

    def update(self, version_type: str):
        if version_type == 'major':
            self._major += 1
            self._minor = 0
            self._patch = 0
        elif version_type == 'minor':
            self._minor += 1
            self._patch = 0
        elif version_type == 'patch':
            self._patch += 1
        else:
            raise Exception("Unkown version type {}.".format(version_type))


def version_type(string: str):
    types = ['major', 'minor', 'patch']
    if string not in types:
        msg = 'Not valid. Must be one of the following values: {}'.format(
            types)
        raise argparse.ArgumentTypeError(msg)
    return string


def cli_parse_args():
    parser = argparse.ArgumentParser(description='Update rust version')
    parser.add_argument('type', type=version_type)
    opts = parser.parse_args()
    return opts.type


def main():
    update_type = cli_parse_args()
    print('Update "{}"'.format(update_type))
    version = read_current_version()
    print('Current version is {}'.format(version))
    version.update(update_type)
    print('Updated to version {}'.format(version))
    update_version_in_files(str(version))
    print('Add tag and push via git.')
    git_push_tag(version)


def git_push_tag(version: str):
    subprocess.run(["git", "add", "ntest/Cargo.toml"])
    subprocess.run(["git", "add", "ntest_test_cases/Cargo.toml"])
    subprocess.run(["git", "tag",
                    "-a", "v{}".format(version),
                    "-m Version {}".format(version)])
    subprocess.run(["git", "commit"])
    subprocess.run(["git", "push", "origin", "v{}".format(version)])


def read_current_version():
    ntest_toml_path = os.path.join(FILE_DIR, 'ntest', 'Cargo.toml')
    with open(ntest_toml_path, 'r') as toml_file:
        toml_content = toml.loads(toml_file.read())
        version = Version(toml_content['package']['version'])
    return version


def update_version_in_files(version: str):
    ntest_toml_path = os.path.join(FILE_DIR, 'ntest', 'Cargo.toml')
    with open(ntest_toml_path, 'r') as toml_file:
        toml_content = toml.loads(toml_file.read())
        toml_content['package']['version'] = version
        toml_content['dependencies']['ntest_test_cases']['version'] = version
        toml_content['dev-dependencies']['ntest_test_cases']['version'] = version
    with open(ntest_toml_path, 'w') as toml_file:
        toml_file.write(toml.dumps(toml_content))
    ntest_test_cases_toml_path = os.path.join(
        FILE_DIR, 'ntest_test_cases', 'Cargo.toml')
    with open(ntest_test_cases_toml_path, 'r') as toml_file:
        toml_content = toml.loads(toml_file.read())
        toml_content['package']['version'] = version
    with open(ntest_test_cases_toml_path, 'w') as toml_file:
        toml_file.write(toml.dumps(toml_content))


if __name__ == "__main__":
    main()
