#lang racket

(require images/flomap
         memo)

(define debug (make-parameter #f))
(define debug-image (make-parameter #f))
(define serial (make-parameter 0))

(command-line
 #:once-each
 [("--debug") "Print debug information" (debug #t)]
 [("--debug-image") "Save a debug image to fuel.png" (debug-image #t)]
 [("--serial") n "Grid serial number" (serial (string->number n))])

; Calculate the fuel at a point
(define/memoize (fuel x y)
  (- (quotient (remainder (* (+ (* (+ x 10) y) (serial)) (+ x 10)) 1000) 100) 5))

; Render a fuel map
(define (render-fuel)
  (flomap->bitmap
   (build-flomap*
    1 300 300
    (λ (x y)
      (vector (/ (+ 5 (fuel x y)) 10))))))

(when (debug-image) 
  (send (render-fuel) save-file "fuel.png" 'png))

; Calculate total fuel at a 3x3 point (from top left)
(define (fuel-3x3 x y)
  (for*/sum ([xd (in-range 3)]
             [yd (in-range 3)])
    (fuel (+ x xd) (+ y yd))))

; DEBUG: Print a 3x3 grid (from top left)
(define (display-3x3 x y)
  (for ([xd (in-range 3)])
    (for ([yd (in-range 3)])
      (printf "~a " (fuel (+ x xd) (+ y yd))))
    (newline)))

; Part 1, maximum fuel on a 3x3 grid
(let ()
  (define-values (x y f)
    (for*/fold ([max-x 0] [max-y 0] [max-fuel (fuel-3x3 0 0)])
               ([x (in-range 298)]
                [y (in-range 298)]
                [f (in-value (fuel-3x3 x y))]
                #:when (> f max-fuel))
  
      (when (debug)
        (printf "new maximum of ~a at ~a,~a\n" f x y))
  
      (values x y f)))

  (printf "[part1] ~a,~a has ~a fuel\n" x y f))

; Calculate total fuel at any size square (from top left)
(define (fuel-square x y [size 3])
  (for*/sum ([xd (in-range size)]
             [yd (in-range size)])
    (fuel (+ x xd) (+ y yd))))

; Part 2: Find maximum fuel at any size
(let ()
  (define-values (x y s f)
    (for*/fold ([max-x 0] [max-y 0] [max-s 1] [max-fuel (fuel-square 0 0 1)])
               ([s (in-range 1 300)]
                [x (in-range (- 301 s))]
                [y (in-range (- 301 s))]
                [f (in-value (fuel-square x y s))]
                #:when (> f max-fuel))
  
      (when (debug)
        (printf "new maximum of ~a at ~a,~a with size ~a\n" f x y s))
      
      (values x y s f)))

  (printf "~a,~a (size: ~a) has ~a fuel\n" x y s f))
