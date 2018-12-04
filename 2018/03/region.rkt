#lang racket

(provide (struct-out region)
         read-regions)

(struct region (id left top width height) #:transparent)

(define (read-region [in (current-input-port)])
  (define line (read-line in))
  (cond
    [(eof-object? line) line]
    [else
     (apply region
            (map string->number
                 (rest (regexp-match #px"#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)"
                                     line))))]))

(define (read-regions [in (current-input-port)])
  (port->list read-region in))