#lang racket

(define (count-letters word)
  (for/fold ([count (hash)])
            ([letter (in-string word)])
    (hash-update count
                 letter
                 add1
                 0)))

(define-values (2s 3s) 
  (for/fold ([2s 0] [3s 0])
            ([line (in-lines)])
    (define counts (count-letters (string-trim line)))
    (values (+ 2s (if (member 2 (hash-values counts)) 1 0))
            (+ 3s (if (member 3 (hash-values counts)) 1 0)))))

(* 2s 3s)
