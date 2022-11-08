import toml
from pathlib import Path
import os
from enum import Enum
import argparse
import subprocess
import time
import sys

FILE_DIR = os.path.dirname(os.path.realpath(__file__))


class Version:

    def __init__(self, version: str):
        splited = version.split('.')
        if len(splited) != 3:
            raise Exception(
                'Unknown version string {} Must be of format <major>.<minor>.<patch>.'.format(version))
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
            raise Exception("Unknown version type {}.".format(version_type))


def version_type(string: str):
    types = ['major', 'minor', 'patch']
    if string not in types:
        msg = 'Not valid. Must be one of the following values: {}'.format(
            types)
        raise argparse.ArgumentTypeError(msg)
    return string


def cli_parse_args():
    parser = argparse.ArgumentParser(description='Publish new rust version')
    parser.add_argument('type', type=version_type)
    opts = parser.parse_args()
    return opts.type


def main():
    update_type = cli_parse_args()
    print('Update "{}"'.format(update_type))
    version = read_current_version()
    print('Current version is {}'.format(version))
    version.update(update_type)
    print('Update to version {}? Press y to continue'.format(version))
    char = sys.stdin.read(1)
    if char == 'y':
        update_version_in_files(str(version))
        deploy_crate()
        print('Add tag and push via git.')
        git_push_with_tag(version)

def deploy_crate():
    # TODO wait till new package version was published
    timout = 5
    subprocess.run(["cargo", "publish", "--verbose" ,"--manifest-path", "ntest_test_cases/Cargo.toml", "--allow-dirty"])
    subprocess.run(["cargo", "publish", "--verbose" ,"--manifest-path", "ntest_timeout/Cargo.toml", "--allow-dirty"])
    print('Wait {} seconds before the main lib will be published'.format(timout))
    time.sleep(timout)
    subprocess.run(["cargo", "publish", "--verbose" ,"--manifest-path", "ntest/Cargo.toml", "--allow-dirty"])

def git_push_with_tag(version: str):
    subprocess.run(["git", "add", "ntest/Cargo.toml"])
    subprocess.run(["git", "add", "ntest_test_cases/Cargo.toml"])
    subprocess.run(["git", "add", "ntest_timeout/Cargo.toml"])
    subprocess.run(["git", "tag",
                    "-a", "v{}".format(version),
                    "-m Version {}".format(version)])
    subprocess.run(["git", "commit", '-m "Release {}"'.format(version)])
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
        toml_content['dependencies']['ntest_timeout']['version'] = version
    with open(ntest_toml_path, 'w') as toml_file:
        toml_file.write(toml.dumps(toml_content))
    
    ntest_test_cases_toml_path = os.path.join(
        FILE_DIR, 'ntest_test_cases', 'Cargo.toml')
    with open(ntest_test_cases_toml_path, 'r') as toml_file:
        toml_content = toml.loads(toml_file.read())
        toml_content['package']['version'] = version
    with open(ntest_test_cases_toml_path, 'w') as toml_file:
        toml_file.write(toml.dumps(toml_content))
    
    ntest_timeout_toml_path = os.path.join(
        FILE_DIR, 'ntest_timeout', 'Cargo.toml')
    with open(ntest_timeout_toml_path, 'r') as toml_file:
        toml_content = toml.loads(toml_file.read())
        toml_content['package']['version'] = version
    with open(ntest_timeout_toml_path, 'w') as toml_file:
        toml_file.write(toml.dumps(toml_content))


if __name__ == "__main__":
    main()
