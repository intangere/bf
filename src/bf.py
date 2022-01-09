import sys

def run(program, tape, data_ptr):

      prog_ptr = 0

      while prog_ptr < len(program):
          op = program[prog_ptr]
          if op == '>':
             data_ptr += 1
          elif op == '<':
             data_ptr -= 1
          elif op == '+':
             tape[data_ptr] += 1
          elif op == '-':
             tape[data_ptr] -= 1
          elif op == '.':
             print(chr(tape[data_ptr]), end='')
          prog_ptr += 1
      return tape, data_ptr

if __name__ == '__main__':

   if len(sys.argv) < 2:
      print('Filename missing')
      sys.exit(1)

   with open(sys.argv[1]) as f:
      program = ''.join([_.strip() for _ in f.readlines()])
      print(program)

      loops = []
      data_ptr = 0
      prog_ptr = 0
      tape = []

      for _ in range(30000):
          tape.append(0)

      while prog_ptr < len(program):

        op = program[prog_ptr]
#       print('running', op)

        if op != '[':
           tape, data_ptr = run(op, tape, data_ptr)
        else:

           if tape[data_ptr] == 0:
              continue

           sub_program = ''
           while op != ']':
             sub_program += op
             prog_ptr += 1
             op = program[prog_ptr]

           while tape[data_ptr] != 0:
              print('running sub program', sub_program)
              tape, data_ptr = run(sub_program, tape, data_ptr)

        prog_ptr += 1

#          print(prog_ptr)
#          print(loops)
