import sys
import psycopg2
with open(sys.argv[1]) as data_file:
    data = data_file.read()
    