#lang racket

(require "circular-list.rkt")
(require racket/trace)

(define debug (make-parameter #f))
(define players (make-parameter 9))
(define last-marble (make-parameter 26))

(command-line
 #:once-each
 [("--debug")
  "Print debug output while solving the problem"
  (debug #t)]
 #:multi
 [("--players")
  n
  "Number of plays"
  (players (string->number n))]
 [("--stop-at")
  n
  "Stop after marble n is played"
  (last-marble (string->number n))])

(let loop ([players (list->circular-list (range 1 (add1 (players))))]
           [scores (hash)]
           [table (list->circular-list '(0))]
           [marble 1])

  (when (debug)
    (printf "[~a] [head:~a] ~a\n"
            (circular-list-peek players)
            (circular-list-peek (circular-list-rotate table))
            (circular-list-rotate-until table zero?)))
  
  (cond
    ; Used up the last marble, output scores
    [(= marble (add1 (last-marble)))
     (apply max (hash-values scores))]
    ; Marbles divisible by 23 scores 23 + removes the marble 7 ago
    [(= 0 (remainder marble 23))
     (let* ([table (circular-list-rotate table -8)]
            [score (+ (circular-list-peek table) marble)]
            [table (circular-list-pop table)]
            [table (circular-list-rotate table 1)])
       (loop (circular-list-rotate players)
             (hash-update scores (circular-list-peek players) (curry + score) 0)
             table
             (add1 marble)))]
    ; All other marbles skip 1 and insert the marble
    [else
     (loop (circular-list-rotate players)
           scores
           (circular-list-push (circular-list-rotate table) marble)
           (add1 marble))]))
           