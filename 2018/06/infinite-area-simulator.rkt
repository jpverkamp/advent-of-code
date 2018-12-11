#lang racket

(require images/flomap
         racket/cmdline)

(define debug-print (make-parameter #f))
(define debug-image (make-parameter #f))
(define part-2-range (make-parameter 10000))
(command-line
 #:once-each
 [("--debug-print")
  "Print debug output while solving the problem"
  (debug-print #t)]
 #:multi
 [("--debug-image")
  filename
  "Write a debug image to <filename>"
  (debug-image filename)]
 [("--range")
  range
  "Set the range for part 2 (default 10,000)"
  (part-2-range (string->number range))])
  
(struct point (x y) #:transparent)

; Read constant POINTS for the current input
(define (read-points [in (current-input-port)])
  (for/list ([line (in-lines in)])
    (match-define (regexp #px"(\\d+), (\\d+)" (list _ raw-x raw-y)) line)
    (define x (string->number raw-x))
    (define y (string->number raw-y))
    (point x y)))

(define POINTS (read-points))

; Determine the bounds for the given points
(define-values (MIN-X MAX-X MIN-Y MAX-Y)
  (for/fold ([min-x +inf.0] [max-x -inf.0] [min-y +inf.0] [max-y -inf.0])
            ([pt (in-list POINTS)])
    (values (min min-x (point-x pt))
            (max max-y (point-x pt))
            (min min-x (point-y pt))
            (max max-y (point-y pt)))))

; If a region has more points than are contained in the bounds, it's infinite
(define VOLUME (* (- MAX-X MIN-X) (- MAX-Y MIN-Y)))

; DEBUG: Assign a (sequential) chinese character to each point; use that to print a debug printout
(define (name-for pt)
  (or 
   (for/first ([i (in-naturals)]
               [pt^ (in-list POINTS)]
               #:when (equal? pt pt^))
     (integer->char (+ #x4E00 i 1)))
   (integer->char #x4E00)))

(define (print-debug)
  (for ([y (in-range (- MIN-Y 1) (+ MAX-Y 2))])
    (for ([x (in-range (- MIN-X 1) (+ MAX-X 2))])
      (printf "~a" (name-for (closest (point x y)))))
    (newline)))

; DEBUG: Assign a random but consistent color to each point; use that to print a debug image
(define color-for
  (let ([colors (for/list ([pt (in-list POINTS)]) (vector (random) (random) (random)))])
    (λ (pt)
      (define closest-pt (closest pt))
      (or 
       (for/first ([color (in-list colors)]
                   [pt^ (in-list POINTS)]
                   #:when (equal? closest-pt pt^))
         color)
       (vector 0 0 0)))))

(define (write-image-debug filename)
  (send
   (flomap->bitmap
    (build-flomap*
     3 (exact-round (- MAX-X MIN-X)) (exact-round (- MAX-Y MIN-Y))
     (λ (x y) (color-for (point (+ x MIN-X) (+ y MIN-Y))))))
   save-file
   filename
   'png))

; Manhattan distance
(define (distance p1 p2)
  (+ (abs (- (point-x p1) (point-x p2)))
     (abs (- (point-y p1) (point-y p2)))))

; Return the point in pts closest to the given point pt
(define (closest target)
  (define-values (min-point min-distance)
    (for*/fold ([min-point #f] [min-distance +inf.0])
               ([pt (in-list POINTS)]
                [d (in-value (distance target pt))]
                #:when (<= d min-distance))
      (values (if (= d min-distance) #f pt) d)))
  min-point)

; The four neighbors of a given point
(define (neighbors target)
  (match-define (point x y) target)
  (list (point x (- y 1))
        (point (+ x 1) y)
        (point x (+ y 1))
        (point (- x 1) y)))

; Calculate the number of points closest to this point than any other via floodfill
(define (area target)
  (let loop ([area 0]
             [to-check (list target)]
             [checked (set)])
    (cond
      [(null? to-check) area]
      [else
       (match-define (list-rest current-to-check next-to-check) to-check)
       (define next-checked (set-add checked current-to-check))
       (cond
         ; Already checked this point, ignore
         [(set-member? checked current-to-check)
          (loop area next-to-check checked)]
         ; More than the maximum area, has gone infinite
         [(> area VOLUME)
          +inf.0]
         ; Closest to target, add and expand
         [(equal? target (closest current-to-check))
          (loop (add1 area)
                (append (neighbors current-to-check) next-to-check)
                next-checked)]
         ; Not closest, don't add or expand
         [else
          (loop area next-to-check next-checked)])])))

(when (debug-image)
  (write-image-debug (debug-image)))

; Find the largest non-infinite area
(printf "[part1]\n")
(for/fold ([max-point #f] [max-area -inf.0])
          ([pt (in-list POINTS)])
  (define a (area pt))
  (when (debug-print)
    (printf "~a (~a) has area: ~a\n" pt (name-for pt) a))
  
  (cond
    [(and (not (infinite? a))
          (> a max-area))
     (when (debug-print) (printf "NEW MAXIMUM!"))
     (values pt a)]
    [else
     (values max-point max-area)]))

(printf "\n[part2]\n")

; Find the center point and flood fill out to all points with X of all points
(define (points-within-range range)
  (let loop ([to-check (set (point (exact-round (/ (+ MIN-X MAX-X) 2))
                                   (exact-round (/ (+ MIN-X MAX-X) 2))))]
             [checked (set)]
             [region (set)])
    
    (when (debug-print) 
      (printf "~a checked, ~a currently to check, next: ~a, region: ~a\n"
              (set-count checked)
              (set-count to-check)
              (if (set-empty? to-check) to-check (set-first to-check))
              #f #;region))

    (cond
      ; Base case: checked all points, return region 
      [(set-empty? to-check) region]
      ; Already checked this point, ignore
      [(set-member? checked (set-first to-check))
       (loop (set-rest to-check) checked region)]
      ; Sum of distances is less than range, include and expand search
      [(< (for/sum ([pt (in-list POINTS)])
            (distance (set-first to-check) pt))
          range)
       (loop (set-union (for/set ([neighbor (in-list (neighbors (set-first to-check)))]
                                  #:when (not (set-member? checked neighbor)))
                          neighbor)
                        (set-rest to-check))
             (set-add checked (set-first to-check))
             (set-add region (set-first to-check)))]
      ; Not in region, skip
      [else
       (loop (set-rest to-check) (set-add checked (set-first to-check)) region)])))

(printf "~a are within ~a\n"
        (set-count (points-within-range (part-2-range)))
        (part-2-range))
                  

