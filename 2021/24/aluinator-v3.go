package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type state struct {
	w int
	x int
	y int
	z int
}

func (s state) get(r string) int {
	if r == "w" {
		return s.w
	} else if r == "x" {
		return s.w
	} else if r == "y" {
		return s.y
	} else if r == "z" {
		return s.z
	} else {
		result, err := strconv.Atoi(r)
		if err != nil {
			log.Panic(err)
		}
		return result
	}
}

func (s state) set(r string, v int) state {
	result := s
	if r == "w" {
		result.w = v
	} else if r == "x" {
		result.x = v
	} else if r == "y" {
		result.y = v
	} else if r == "z" {
		result.z = v
	}
	return result
}

type input struct {
	min string
	max string
}

func minput(i1 input, i2 input) input {
	result := input{"", ""}

	if i1.min < i2.min { 
		result.min = i1.min
	} else {
		result.min = i2.min
	}

	if i1.max > i2.max { 
		result.max = i1.max
	} else {
		result.max = i2.max
	}

	return result
}

type Op func(int, int) int

func is_reg(s string) bool {
	return s == "w" || s == "x" || s == "y" || s == "z"
}

func main() {
	// Store a list of operators
	operators := make(map[string]Op)
	operators["add"] = func(a int, b int) int { return a + b }
	operators["mul"] = func(a int, b int) int { return a * b }
	operators["div"] = func(a int, b int) int { if b == 0 { return 0 } else { return a / b } }
	operators["mod"] = func(a int, b int) int { if b == 0 { return 0 } else { return a % b } }
	operators["eql"] = func(a int, b int) int { if a == b { return 1 } else { return 0 } }

	// Open the input file
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Initialize the register states
	var states map[state]input
	var new_states map[state]input

	states = make(map[state]input)
	states[state{0, 0, 0, 0}] = input{"", ""}

	// Start scanning through the file line by line
	scanner := bufio.NewScanner(file)
	input_length := 0
	line_number := 0
	for scanner.Scan() {
		line_number += 1
		line := scanner.Text()
		fields := strings.Fields(line)
		log.Printf("[%d len=%d states=%d] %s", line_number, input_length, len(states), line)

		// Build a new set of states from the current set
		new_states = make(map[state]input)
		
		// An input value adds 9 new states for each old state, each possible non-zero input
		if fields[0] == "inp" {
			a := fields[1]
			for i := 1; i <= 9; i++ {
				for old_state, old_input := range states {
					// Update the proper variable in the input string
					new_state := old_state.set(a, i)
					
					// Calculate the new minimum/maximum input string
					new_input := input{fmt.Sprint(old_input.min, i), fmt.Sprint(old_input.max, i)}
						
					// Check if we've already stored that in our new_state, if so keep the min/max
					if prev_new_input, ok := new_states[new_state]; ok {
						new_states[new_state] = minput(prev_new_input, new_input)
					} else {
						new_states[new_state] = new_input
					}

					// log.Printf("  %v -> %v", new_state, new_states[new_state])
				}
			}

			
		} else {
			// Fetch the proper operator function
			f, ok := operators[fields[0]]
			if !ok {
				log.Fatal("Unknown operator", fields[1])
			}
			a := fields[1]

			// Update all old states
			for old_state, old_input := range states {
				b := fields[2]

				a_val := old_state.get(a)
				b_val := old_state.get(b)
				r_val := f(a_val, b_val)

				// log.Printf("  %d %s %d = %d -> %s", a_val, fields[0], b_val, r_val, a)

				new_state := old_state.set(a, r_val)

				// Check if we've already stored that in our new_state, if so keep the min/max
				if prev_new_input, ok := new_states[new_state]; ok {
					new_states[new_state] = minput(prev_new_input, old_input)
				} else {
					new_states[new_state] = old_input
				}

				// log.Printf("  %v -> %v", new_state, new_states[new_state])
			}
		}

		// Finally, swap new_state to state
		states = new_states

		// DEBUG
		/*
		if line_number > 10 {
			break
		}
		*/
	}

	result := input{"", ""}
	result_set := false

	for state, input := range states {
		if state.z != 0 {
			continue
		}

		if !result_set {
			result = input
			result_set = true
			continue
		}

		result = minput(result, input)
	}

	fmt.Println(result)
}
