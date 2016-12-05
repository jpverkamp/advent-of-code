def generate_checksum(name):
    '''
    Custom checksum function by sorting all characters in the input on a tuple
    of: length (shortest first) and the letter itself for alphabetical sorting.
    '''

    return ''.join(list(sorted(
        set(name) - {'-'},
        key = lambda letter : (
            -name.count(letter),
            letter
        )
    )))[:5]

def decrypt(name, key):
    '''Shift all characters in the name by key positions.'''

    offset = ord('a')
    def shift(c):
        if c == '-':
            return ' '
        else:
            return chr((ord(c) - offset + key) % 26 + offset)

    return ''.join(map(shift, name))
