
CC=gcc
CFLAGS=-O0 -g -D_GNU_SOURCE -Werror-implicit-function-declaration

all: awk

# awk: awk.o compare_string_array.o endofname.o full_write.o getopt32.o individual.o llist.o perror_msg.o process_escape_sequence.o ptr_to_globals.o read_printf.o read.o skip_whitespace.o verror_msg.o wfopen_input.o wfopen.o xfunc_die.o xfuncs_printf.o xfuncs.o xregcomp.o safe_strncpy.o xatonum.o
# 	$(CC) $(CFLAGS) -lm -o awk *.o
# make a library libawk.a
awk: awk.o compare_string_array.o endofname.o full_write.o getopt32.o individual.o inc_stripped.o llist.o perror_msg.o process_escape_sequence.o ptr_to_globals.o read_printf.o read.o skip_whitespace.o verror_msg.o wfopen_input.o wfopen.o xfunc_die.o xfuncs_printf.o xfuncs.o xregcomp.o safe_strncpy.o xatonum.o
	ar rcs libawk.a awk.o compare_string_array.o endofname.o full_write.o getopt32.o individual.o inc_stripped.o llist.o perror_msg.o process_escape_sequence.o ptr_to_globals.o read_printf.o read.o skip_whitespace.o verror_msg.o wfopen_input.o wfopen.o xfunc_die.o xfuncs_printf.o xfuncs.o xregcomp.o safe_strncpy.o xatonum.o
	nm libawk.a > libawk.a.nm

inc_stripped.o: inc_stripped.c
	$(CC) $(CFLAGS) -c inc_stripped.c

individual.o: individual.c
	$(CC) $(CFLAGS) -c individual.c

awk.o: awk.c
	$(CC) $(CFLAGS) -c awk.c

compare_string_array.o: compare_string_array.c
	$(CC) $(CFLAGS) -c compare_string_array.c

endofname.o: endofname.c
	$(CC) $(CFLAGS) -c endofname.c

full_write.o: full_write.c
	$(CC) $(CFLAGS) -c full_write.c

getopt32.o: getopt32.c
	$(CC) $(CFLAGS) -c getopt32.c

llist.o: llist.c
	$(CC) $(CFLAGS) -c llist.c

perror_msg.o: perror_msg.c
	$(CC) $(CFLAGS) -c perror_msg.c

process_escape_sequence.o: process_escape_sequence.c
	$(CC) $(CFLAGS) -c process_escape_sequence.c

ptr_to_globals.o: ptr_to_globals.c
	$(CC) $(CFLAGS) -c ptr_to_globals.c

read_printf.o: read_printf.c
	$(CC) $(CFLAGS) -c read_printf.c

read.o: read.c
	$(CC) $(CFLAGS) -c read.c

skip_whitespace.o: skip_whitespace.c
	$(CC) $(CFLAGS) -c skip_whitespace.c

verror_msg.o: verror_msg.c
	$(CC) $(CFLAGS) -c verror_msg.c

wfopen_input.o: wfopen_input.c
	$(CC) $(CFLAGS) -c wfopen_input.c

wfopen.o: wfopen.c
	$(CC) $(CFLAGS) -c wfopen.c

xfunc_die.o: xfunc_die.c
	$(CC) $(CFLAGS) -c xfunc_die.c

xfuncs_printf.o: xfuncs_printf.c
	$(CC) $(CFLAGS) -c xfuncs_printf.c

xfuncs.o: xfuncs.c
	$(CC) $(CFLAGS) -c xfuncs.c

xregcomp.o: xregcomp.c
	$(CC) $(CFLAGS) -c xregcomp.c

safe_strncpy.o: safe_strncpy.c
	$(CC) $(CFLAGS) -c safe_strncpy.c

xatonum.o: xatonum.c
	$(CC) $(CFLAGS) -c xatonum.c

clean:
	rm -f awk *.o

clean_all:
	rm -f awk *.o *.a