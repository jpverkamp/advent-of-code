#lang racket

(struct state (data length elf1 elf2) #:transparent)

(define (display-state s)
  (match-define (state data length elf1 elf2) s)
  (for ([i (in-range length)])
    (printf (cond
              [(= i elf1) "(~a) "]
              [(= i elf2) "[~a] "]
              [else       " ~a  "])
            (hash-ref data i)))
  (newline))

(define (initial-state)
  (state (hash 0 3 1 7)
         2
         0
         1))

; Add a single value to the end of the current state
(define (add-recipe s v)
  (match-define (state data length elf1 elf2) s)
  (state (hash-set data length v)
         (add1 length)
         elf1
         elf2))

; Add one or two recipes to the state
(define (add-recipes s)
  (match-define (state data length elf1 elf2) s)
  (define sum (+ (hash-ref data elf1) (hash-ref data elf2)))
  (cond
    [(>= sum 10)
     (add-recipe (add-recipe s (quotient sum 10)) (remainder sum 10))]
    [else
     (add-recipe s sum)]))

; Update each elf's current recipe
(define (move-elfs s)
  (match-define (state data length elf1 elf2) s)
  (state data
         length
         (remainder (+ elf1 (hash-ref data elf1) 1) length)
         (remainder (+ elf2 (hash-ref data elf2) 1) length)))

; Do a full update
(define (tick s)
  (move-elfs (add-recipes s)))

; Calculate the score after a coordinate
(define (score i)
  (define state
    (let loop ([s (initial-state)])
      (cond
        [(< (state-length s) (+ i 10))
         (loop (tick s))]
        [else s])))
  (for/list ([i (in-range i (+ i 10))])
    (hash-ref (state-data state) i)))

; Find a specific pattern in the input stream
(define (search ls)
  (let loop ([state (initial-state)]
             [index 0]
             [to-find ls])
    (when (zero? (remainder index 1000)) (displayln index))
    (cond
      ; Found our target, return the index it started at
      [(null? to-find) (- index (length ls))]
      ; Don't have enough data, generate some more
      [(>= index (state-length state))
       (loop (tick state) index to-find)]
      ; The current match continues
      [(equal? (first to-find) (hash-ref (state-data state) index))
       (loop state (add1 index) (rest to-find))]
      ; The current match does not continue, reset to where we started + 1
      [else
       (loop state (+ (- index (length ls)) (length to-find) 1) ls)])))

; Helper functions to convert between numbers and lists of digits
(define (int->digits n)
  (let loop ([n n] [digits '()])
    (cond
      [(< n 10) (list* n digits)]
      [else (loop (quotient n 10)
                  (list* (remainder n 10) digits))])))

(define (digits->int ls)
  (let loop ([n 0] [digits ls])
    (cond
      [(null? digits) n]
      [else
       (loop (+ (* n 10) (first digits))
             (rest digits))])))

; Find score/search for any given values
(define argv (current-command-line-arguments))
(for ([arg (in-vector argv)])
  (printf "input: ~a\n" arg)
  (printf "[part1] ~a\n" (digits->int (score (string->number arg))))
  (printf "[part2] ~a\n" (search (int->digits (string->number arg)))))