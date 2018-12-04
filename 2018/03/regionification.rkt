#lang racket

(require "region.rkt")

(define regions-per-point
  (for*/fold ([count (hash)])
             ([r (in-list (read-regions))]
              [x (in-range (region-left r) (+ (region-left r) (region-width r)))]
              [y (in-range (region-top r) (+ (region-top r) (region-height r)))])
    (hash-update count (list x y) add1 0)))

(length
 (filter (λ (v) (> v 1))
         (hash-values regions-per-point)))
