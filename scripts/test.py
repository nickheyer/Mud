from os import path

print('ALL IS WELL - Python <3')

with open(path.join(path.dirname(path.realpath(__file__)), 'ITWORKED.txt'), 'w') as fp:
  fp.write("HELL YEAH BROTHER WE ARE IN BUSINESS")