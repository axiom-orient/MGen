#!/usr/bin/env python3
"""
TMAndroid Pre-generation Hook

이 스크립트는 Cookiecutter가 프로젝트를 생성하기 전에 실행됩니다.
입력 값을 검증하고 잘못된 설정이 있으면 프로젝트 생성을 중단합니다.
"""

import re
import sys

# Cookiecutter 변수
PROJECT_NAME = "{{ cookiecutter.project_name }}"
PROJECT_SLUG = "{{ cookiecutter.project_slug }}"
PACKAGE_NAME = "{{ cookiecutter.package_name }}"
BASE_PACKAGE = "{{ cookiecutter.base_package }}"
MIN_SDK = "{{ cookiecutter.min_sdk }}"


def print_error(message: str) -> None:
    """에러 메시지 출력"""
    print(f"\n❌ ERROR: {message}\n", file=sys.stderr)


def validate_project_name() -> None:
    """프로젝트 이름 검증"""
    if not PROJECT_NAME:
        print_error("Project name cannot be empty")
        sys.exit(1)

    if len(PROJECT_NAME) > 50:
        print_error("Project name too long (max 50 characters)")
        sys.exit(1)

    # 첫 문자는 반드시 알파벳이어야 함 (Kotlin 식별자 규칙)
    if not re.match(r'^[A-Za-z]', PROJECT_NAME):
        print_error("Project name must start with a letter (A-Z or a-z)")
        sys.exit(1)

    # 특수문자 검증 (알파벳, 숫자, 공백, 하이픈, 언더스코어만 허용)
    if not re.match(r'^[A-Za-z][A-Za-z0-9\s_-]*$', PROJECT_NAME):
        print_error("Project name must start with a letter and can only contain letters, numbers, spaces, hyphens, and underscores")
        sys.exit(1)


def validate_package_name() -> None:
    """패키지명 검증"""
    # package_name은 소문자 알파벳과 숫자만 허용
    if not re.match(r'^[a-z][a-z0-9]*$', PACKAGE_NAME):
        print_error(
            f"Invalid package name '{PACKAGE_NAME}'\n"
            "Package name must start with lowercase letter and contain only lowercase letters and digits"
        )
        sys.exit(1)

    # Java 예약어 체크
    java_keywords = {
        'abstract', 'continue', 'for', 'new', 'switch', 'assert', 'default', 'goto', 'package',
        'synchronized', 'boolean', 'do', 'if', 'private', 'this', 'break', 'double', 'implements',
        'protected', 'throw', 'byte', 'else', 'import', 'public', 'throws', 'case', 'enum',
        'instanceof', 'return', 'transient', 'catch', 'extends', 'int', 'short', 'try', 'char',
        'final', 'interface', 'static', 'void', 'class', 'finally', 'long', 'strictfp', 'volatile',
        'const', 'float', 'native', 'super', 'while'
    }

    if PACKAGE_NAME in java_keywords:
        print_error(f"Package name '{PACKAGE_NAME}' is a Java reserved keyword")
        sys.exit(1)


def validate_base_package() -> None:
    """베이스 패키지 검증"""
    # 패키지명 규칙: com.example.app 형태
    package_pattern = r'^[a-z][a-z0-9]*(\.[a-z][a-z0-9]*)+$'

    if not re.match(package_pattern, BASE_PACKAGE):
        print_error(
            f"Invalid base package '{BASE_PACKAGE}'\n"
            "Base package must follow Java package naming conventions (e.g., com.example.app)"
        )
        sys.exit(1)

    # 최소 2개 세그먼트 필요 (예: com.example)
    segments = BASE_PACKAGE.split('.')
    if len(segments) < 2:
        print_error("Base package must have at least 2 segments (e.g., com.example)")
        sys.exit(1)


def validate_sdk_version() -> None:
    """SDK 버전 검증"""
    try:
        min_sdk_int = int(MIN_SDK)
        if min_sdk_int < 21:
            print_error(f"Minimum SDK {min_sdk_int} is too low. Android minimum supported is 21 (Lollipop)")
            sys.exit(1)
        if min_sdk_int > 35:
            print_error(f"Minimum SDK {min_sdk_int} is higher than current target (35)")
            sys.exit(1)
    except ValueError:
        print_error(f"Invalid minimum SDK version '{MIN_SDK}'. Must be a number.")
        sys.exit(1)


def print_info() -> None:
    """검증 통과 후 정보 출력"""
    print("\n" + "=" * 60)
    print("🚀 TMAndroid - Project Generation")
    print("=" * 60)
    print(f"Project Name:     {PROJECT_NAME}")
    print(f"Project Slug:     {PROJECT_SLUG}")
    print(f"Base Package:     {BASE_PACKAGE}")
    print(f"Min SDK:          {MIN_SDK}")
    print(f"Firebase:         {{ cookiecutter.use_firebase }}")
    print(f"Architecture:     {{ cookiecutter.architecture_pattern }}")
    print("=" * 60)
    print("✅ Validation passed. Generating project...\n")


if __name__ == '__main__':
    try:
        validate_project_name()
        validate_package_name()
        validate_base_package()
        validate_sdk_version()
        print_info()
    except Exception as e:
        print_error(f"Unexpected validation error: {e}")
        sys.exit(1)
