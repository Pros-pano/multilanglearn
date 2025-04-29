# Python Day 1: Print, Variables, and Data Types
# This program demonstrates basic Python concepts including:
# - Print statements
# - Variables
# - All Python data types with their features

# ====================== PRINT STATEMENTS ======================
print("===== PRINT STATEMENTS =====")

# Basic print statement
print("Hello, World!")

# Print multiple items separated by commas (adds spaces automatically)
print("Hello,", "Python", "Learner!")

# Print with custom separator
print("Python", "is", "fun", sep="-")

# Print with custom end character (default is newline)
print("This doesn't end with a newline...", end=" ")
print("See?")

# Formatted string (f-string) - modern way to format output
name = "Python Learner"
print(f"Welcome, {name}!")

# ====================== VARIABLES ======================
print("\n===== VARIABLES =====")

# Variables don't need type declarations
# They are dynamically typed and can change type
x = 10
print(f"x = {x}, type: {type(x)}")

x = "Now I'm a string"
print(f"x = {x}, type: {type(x)}")

# Multiple assignment
a, b, c = 1, 2, 3
print(f"a = {a}, b = {b}, c = {c}")

# Swap variables without a temporary variable
a, b = b, a
print(f"After swap: a = {a}, b = {b}")

# ====================== DATA TYPES ======================
print("\n===== DATA TYPES =====")

# 1. Numeric Types
print("\n-- NUMERIC TYPES --")

# Integer (int) - unlimited precision
integer_number = 42
large_integer = 1234567890123456789012345678901234567890
print(f"Integer: {integer_number}, type: {type(integer_number)}")
print(f"Large integer: {large_integer}")
print(f"Min/Max: Integers in Python 3 have unlimited precision")

# Float - double precision, typically 15-17 significant digits
float_number = 3.14159
scientific_notation = 1.23e-4  # 0.000123
print(f"Float: {float_number}, type: {type(float_number)}")
print(f"Scientific notation: {scientific_notation}")
print(f"Min/Max values: {float('inf')} / {float('-inf')}")
print(f"Special values: NaN (Not a Number): {float('nan')}")

# Complex numbers
complex_number = 3 + 4j
print(f"Complex: {complex_number}, type: {type(complex_number)}")
print(f"Real part: {complex_number.real}, Imaginary part: {complex_number.imag}")
print(f"Conjugate: {complex_number.conjugate()}")

# 2. Boolean Type
print("\n-- BOOLEAN TYPE --")
bool_true = True
bool_false = False
print(f"Boolean values: {bool_true}, {bool_false}, type: {type(bool_true)}")
print(f"Boolean operations: AND: {bool_true and bool_false}, OR: {bool_true or bool_false}, NOT: {not bool_true}")

# 3. Sequence Types
print("\n-- SEQUENCE TYPES --")

# String (str) - immutable sequence of Unicode characters
string_value = "Hello, Python!"
multi_line_string = """This is a
multi-line string"""
print(f"String: {string_value}, type: {type(string_value)}")
print(f"String length: {len(string_value)}")
print(f"String operations: Uppercase: {string_value.upper()}")
print(f"Indexing: {string_value[0]}, {string_value[-1]}")  # First and last character
print(f"Slicing: {string_value[0:5]}")  # First 5 characters
print(f"Multi-line string: {multi_line_string}")

# Raw string (ignores escape characters)
raw_string = r"C:\Users\name\Documents"
print(f"Raw string: {raw_string}")

# List - mutable sequence
my_list = [1, "apple", 3.14, True]
print(f"List: {my_list}, type: {type(my_list)}")
print(f"List length: {len(my_list)}")
print(f"List indexing: {my_list[1]}")
my_list.append("new item")
print(f"After append: {my_list}")
print(f"List comprehension: {[x*2 for x in range(5)]}")

# Tuple - immutable sequence
my_tuple = (1, "apple", 3.14, True)
print(f"Tuple: {my_tuple}, type: {type(my_tuple)}")
print(f"Tuple length: {len(my_tuple)}")
print(f"Tuple indexing: {my_tuple[1]}")
print(f"Single item tuple needs comma: {(1,)}")

# Range - immutable sequence representing a range of numbers
my_range = range(5)  # 0, 1, 2, 3, 4
print(f"Range: {my_range}, type: {type(my_range)}")
print(f"Range as list: {list(my_range)}")
print(f"Range with start, stop, step: {list(range(2, 10, 2))}")

# 4. Mapping Type
print("\n-- MAPPING TYPE --")

# Dictionary (dict) - mutable mapping of keys to values
my_dict = {"name": "Python", "year": 1991, "is_awesome": True}
print(f"Dictionary: {my_dict}, type: {type(my_dict)}")
print(f"Dictionary access: {my_dict['name']}")
print(f"Dictionary get (safe): {my_dict.get('version', 'Not found')}")
my_dict["version"] = 3.10
print(f"After adding a key: {my_dict}")
print(f"Dictionary keys: {list(my_dict.keys())}")
print(f"Dictionary values: {list(my_dict.values())}")
print(f"Dictionary items: {list(my_dict.items())}")

# 5. Set Types
print("\n-- SET TYPES --")

# Set - mutable unordered collection of unique elements
my_set = {1, 2, 3, 2, 1}  # Duplicates are automatically removed
print(f"Set: {my_set}, type: {type(my_set)}")
print(f"Set length: {len(my_set)}")
my_set.add(4)
print(f"After adding element: {my_set}")
print(f"Set operations: Union: {my_set | {5, 6}}, Intersection: {my_set & {3, 4, 5}}")

# Frozen set - immutable version of set
my_frozen_set = frozenset([1, 2, 3, 2, 1])
print(f"Frozen set: {my_frozen_set}, type: {type(my_frozen_set)}")
print(f"Frozen set is immutable and hashable")

# 6. Binary Types
print("\n-- BINARY TYPES --")

# Bytes - immutable sequence of bytes
my_bytes = b"hello"
print(f"Bytes: {my_bytes}, type: {type(my_bytes)}")
print(f"Bytes indexing gives integers: {my_bytes[0]}")

# Bytearray - mutable sequence of bytes
my_bytearray = bytearray(b"hello")
print(f"Bytearray: {my_bytearray}, type: {type(my_bytearray)}")
my_bytearray[0] = 72  # ASCII for 'H'
print(f"After modification: {my_bytearray}")

# Memoryview - memory view of a bytes-like object
my_memoryview = memoryview(b"hello")
print(f"Memoryview: {my_memoryview}, type: {type(my_memoryview)}")
print(f"Memoryview access: {my_memoryview[0]}")

# 7. None Type
print("\n-- NONE TYPE --")
my_none = None
print(f"None value: {my_none}, type: {type(my_none)}")
print(f"None is used to represent the absence of a value")

# 8. Special Collections (from collections module)
print("\n-- SPECIAL COLLECTIONS --")

# Named Tuple
from collections import namedtuple
Point = namedtuple('Point', ['x', 'y'])
p = Point(11, y=22)
print(f"Named tuple: {p}, type: {type(p)}")
print(f"Named tuple access: {p.x}, {p[1]}")

# OrderedDict (note: regular dicts are now ordered by default in Python 3.7+)
from collections import OrderedDict
ordered_dict = OrderedDict([('a', 1), ('b', 2)])
print(f"OrderedDict: {ordered_dict}, type: {type(ordered_dict)}")

# Counter
from collections import Counter
my_counter = Counter("mississippi")
print(f"Counter: {my_counter}, type: {type(my_counter)}")
print(f"Most common: {my_counter.most_common(2)}")

# Deque (double-ended queue)
from collections import deque
my_deque = deque([1, 2, 3])
print(f"Deque: {my_deque}, type: {type(my_deque)}")
my_deque.appendleft(0)
my_deque.append(4)
print(f"After modification: {my_deque}")

# DefaultDict
from collections import defaultdict
my_defaultdict = defaultdict(int)  # Default factory returns 0 for missing keys
my_defaultdict['a'] += 1
print(f"DefaultDict: {dict(my_defaultdict)}, type: {type(my_defaultdict)}")
print(f"Missing key returns: {my_defaultdict['b']}")

# 9. Decimal and Fraction (for precise arithmetic)
print("\n-- DECIMAL AND FRACTION --")

# Decimal - precise decimal arithmetic
from decimal import Decimal
my_decimal = Decimal('0.1')
print(f"Decimal: {my_decimal}, type: {type(my_decimal)}")
print(f"Regular float 0.1 + 0.2: {0.1 + 0.2}")  # Shows floating point precision issues
print(f"Decimal 0.1 + 0.2: {Decimal('0.1') + Decimal('0.2')}")  # Precise

# Fraction - rational numbers
from fractions import Fraction
my_fraction = Fraction(1, 3)
print(f"Fraction: {my_fraction}, type: {type(my_fraction)}")
print(f"Fraction from float: {Fraction(1.5)}")
print(f"Fraction from string: {Fraction('1.5')}")

# 10. Datetime Types
print("\n-- DATETIME TYPES --")

# DateTime, Date, Time
from datetime import datetime, date, time, timedelta
current_datetime = datetime.now()
my_date = date(2025, 2, 24)
my_time = time(13, 30, 0)
print(f"Datetime: {current_datetime}, type: {type(current_datetime)}")
print(f"Date: {my_date}, type: {type(my_date)}")
print(f"Time: {my_time}, type: {type(my_time)}")

# TimeDelta - represents a duration
one_day = timedelta(days=1)
tomorrow = my_date + one_day
print(f"TimeDelta: {one_day}, type: {type(one_day)}")
print(f"Tomorrow: {tomorrow}")

# 11. Enum Type (from enum module)
print("\n-- ENUM TYPE --")

from enum import Enum, auto
class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = auto()  # auto-assigns the next value

print(f"Enum member: {Color.RED}, type: {type(Color.RED)}")
print(f"Enum value: {Color.RED.value}, name: {Color.RED.name}")
print(f"Auto value: {Color.BLUE.value}")

# Type Conversion Examples
print("\n===== TYPE CONVERSION =====")
num_int = 123
num_float = float(num_int)
num_str = str(num_int)
num_bool = bool(num_int)  # Any non-zero number is True
str_list = list("hello")  # Convert string to list of characters

print(f"Original int: {num_int}")
print(f"Converted to float: {num_float}")
print(f"Converted to string: {num_str}")
print(f"Converted to bool: {num_bool}")
print(f"String to list: {str_list}")

# Finding the type and checking if an object is of a certain type
print("\n===== TYPE CHECKING =====")
value = 42
print(f"Type of value: {type(value)}")
print(f"Is integer? {isinstance(value, int)}")
print(f"Is numeric? {isinstance(value, (int, float, complex))}")

# Memory size of objects
print("\n===== OBJECT SIZE =====")
import sys
print(f"Size of integer: {sys.getsizeof(0)} bytes")
print(f"Size of empty string: {sys.getsizeof('')} bytes")
print(f"Size of empty list: {sys.getsizeof([])} bytes")
print(f"Size of empty dict: {sys.getsizeof({})} bytes")