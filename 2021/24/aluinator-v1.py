
state = {
    'w': {0: {}},
    'x': {0: {}},
    'y': {0: {}},
    'z': {0: {}},
}

input_counter = 0
for i, (cmd, a, b) in enumerate(code, 1):
    logging.info(f'[{i:04d}] {cmd} {a} {b}')

    # Take the current input value and add that to the state to get here
    if cmd == 'inp':
        state[a] = {
            digit: {input_counter: {digit}}
            for digit in ALL_INPUTS
        }
        input_counter += 1

    # Otherwise, try to apply the given operation
    # But only on input states that match
    else:
        op = OPS[cmd]

        # If we have a constant, just apply it (path hasn't changed)
        if isinstance(b, int):
            state[a] = {
                op(key, b): key_path
                for key, key_path in state[a].items()
            }

        # Otherwise, apply all changes if the paths are the same
        else:
            new_output = defaultdict(lambda: defaultdict(set))

            for key, key_path in state[a].items():
                logging.debug(f'{PRE}- {key=}, {key_path=}')

                for value, value_path in state[b].items():
                    result = op(key, value)
                    logging.debug(f'{PRE}-- {value=}, {value_path=}, {result=}')

                    all_indexes = key_path.keys() | value_path.keys()

                    # There are no restrictions on inputs at all
                    if not key_path and not value_path:
                        logging.debug(f'{PRE}--- No restrictions')
                        if result not in new_output:
                            new_output[result] = defaultdict(set)

                    # Key path is known but not value path
                    if not value_path:
                        logging.debug(f'{PRE}--- No restriction on value')
                        for i, inputs in key_path.items():
                            new_output[result][i] |= inputs

                    # Value path is known, but not key
                    elif not key_path:
                        logging.debug(f'{PRE}--- No restriction on key')
                        for i, inputs in value_path.items():
                            new_output[result][i] |= inputs

                    # There are no overlapping indexes, include both
                    #elif not any(
                    #    index in key_path and index in value_path
                    #    for index in all_indexes
                    #):
                    #    logging.debug(f'{PRE}--- No overlapping indexes, combine')
                    #    for index in all_indexes:
                    #        new_output[result][index] = key_path.get(index) or value_path.get(index)

                    # The indexes are compatible
                    elif any(
                        index not in key_path or index not in value_path or key_path[index] == value_path[index]
                        for index in all_indexes
                    ):
                        logging.debug(f'{PRE}--- Compatible indexes, combine')
                        for index in all_indexes:
                            new_output[result][index] = key_path.get(index) or value_path.get(index)

                    else:
                        raise NotImplementedError

            new_output = {
                k1: {
                    k2: s
                    for k2, s in new_output[k1].items()
                    if s != ALL_INPUTS
                }
                for k1 in new_output
            }
            logging.debug(f'{PRE}{new_output=}')
            state[a] = new_output

    #logging.info(f'\n{pformat(state)}')

    # DEBUG
    #if i > 25:
    #    break

print()
#pprint(state)

pprint(state['z'])
