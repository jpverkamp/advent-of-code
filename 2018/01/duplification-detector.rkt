#lang racket

(define input (port->list))

(let loop ([seq input]
           [sum 0]
           [seen (set)])
  (cond
    [(null? seq)
     (loop input sum seen)]
    [(set-member? seen (+ (first seq) sum))
     (+ (first seq) sum)]
    [else
     (define new-sum (+ (first seq) sum))
     (loop (rest seq) new-sum (set-add seen new-sum))]))
