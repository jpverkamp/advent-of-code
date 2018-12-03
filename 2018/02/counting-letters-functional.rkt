#lang racket

(define (count-letters word)
  (for/fold ([count (hash)])
            ([letter (in-string word)])
    (hash-update count
                 letter
                 add1
                 0)))

(let loop ([2s 0] [3s 0] [lines (port->lines)])
  (cond
    [(empty? lines) (* 2s 3s)]
    [else
     (define counts (count-letters (string-trim (first lines))))
     (loop (+ 2s (if (member 2 (hash-values counts)) 1 0))
           (+ 3s (if (member 3 (hash-values counts)) 1 0))
           (rest lines))]))