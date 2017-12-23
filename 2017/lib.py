import re

def math(expression, variables):
    '''Safely evaluate a mathematical expression with the given variables.'''

    if re.match(r'[^0-9a-z+\-*/ ]', expression):
        raise Exception('Unsafe expression: {}'.format(expression))

    # TODO: Make this actually safe.

    return eval(expression, globals(), variables)
