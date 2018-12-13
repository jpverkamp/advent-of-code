#lang racket

(define debug (make-parameter #f))
(define generations (make-parameter 20))

(command-line
 #:once-each
 [("--debug") "Print debug information" (debug #t)]
 [("--generations") n "Number of generations to run" (generations (string->number n))])

; Read the initial state from the first line of stdin
(define INITIAL-STATE
  (for/set ([i (in-naturals)]
            [c (in-string (list-ref (string-split (read-line)) 2))]
            #:when (char=? c #\#))
    i))

(define _ (read-line))

; Read the rules from stdin, one per line, until we get them all
(define RULES
  (for/hash ([line (in-lines)])
    (define parts (string-split line))
    (values (list-ref parts 0) (string-ref (list-ref parts 2) 0))))

; Find the current minimum and maximum value
(define (bounds state)
  (values (apply min (set->list state))
          (apply max (set->list state))))

; Count how many plants the current state has
(define (count-plants state)
  (define-values (lo hi) (bounds state))
  (for/sum ([i (in-range lo (add1 hi))])
    (if (set-member? state i) 1 0)))

; Sum the index of all current plants
(define (sum-plant-indexes state)
  (apply + (set->list state)))

; Create a string for the current state
(define (state->string state)
  (define-values (lo hi) (bounds state))
  (list->string
   (for/list ([i (in-range lo (add1 hi))])
     (if (set-member? state i) #\# #\.))))

; Get the context of a point (2 on each side), used to look up RULES
(define (context state x)
  (list->string
   (for/list ([i (in-range (- x 2) (+ x 3))])
     (if (set-member? state i) #\# #\.))))

; Update the state by applying the RULES to each context
(define (update state)
  (define-values (lo hi) (bounds state))
  (for/set ([i (in-range (- lo 2) (+ hi 3))]
            #:when (char=? (hash-ref RULES (context state i) #\.) #\#))
    i))

; Loop to calculate final state
(define FINAL-STATE
  (for/fold ([state INITIAL-STATE])
            ([generation (in-range (+ (generations) 1))])
    
    (when (debug)
      (printf "gen ~a, ~a plants, sum_index = ~a : ~a\n"
              generation
              (count-plants state)
              (sum-plant-indexes state)
              (string-trim (state->string state) #px"\\.*")))
    
    (update state)))

(printf "Final state: ~a plants, sum_index = ~a : ~a\n"
        (count-plants FINAL-STATE)
        (sum-plant-indexes FINAL-STATE)
        (string-trim (state->string FINAL-STATE) #px"\\.*"))

#|
NOTE:
gen 296, 73 plants, sum_index = 21985 : ...
gen 297, 73 plants, sum_index = 22058 : ...
gen 298, 73 plants, sum_index = 22131 : ...
gen 299, 73 plants, sum_index = 22204 : ...
gen 300, 73 plants, sum_index = 22277 : ...

Each generation is stable, adding 73 to the index (moving right by 1) each time.

So the answer for 50000000000 (given input.txt) is:

$ math "(50000000000-300)*73+22277"

3650000000377
|#