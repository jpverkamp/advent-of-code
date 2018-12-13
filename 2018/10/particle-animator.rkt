#lang racket
  
(require images/flomap)

(struct particle (px py vx vy) #:transparent)

(define (particle-update p [steps 1])
  (match-define (particle px py vx vy) p)
  (particle (+ px (* steps vx)) (+ py (* steps vy)) vx vy))
                           
(define (read-particle [in (current-input-port)])
  (define line (read-line))
  (cond
    [(eof-object? line) line]
    [else
     (match-define
       (regexp #px"position=<\\s*(-?\\d+),\\s*(-?\\d+)> velocity=<\\s*(-?\\d+),\\s*(-?\\d+)>"
               (list _ px py vx vy))
       line)
     (particle (string->number px)
               (string->number py)
               (string->number vx)
               (string->number vy))]))

(define INITIAL-PARTICLES
  (port->list read-particle))

; Get the bounds at a given tick
(define (bounds tick)
  (define particles (map (curryr particle-update tick) INITIAL-PARTICLES))
  (for/fold ([min-x +inf.0] [max-x -inf.0] [min-y +inf.0] [max-y -inf.0])
            ([p (in-list particles)])
    (match-define (particle px py vx vy) p)
    (values (min min-x px) (max max-x px) (min min-y py) (max max-y py))))

; Get the volume at a given tick
(define (volume tick)
  (define-values (min-x max-x min-y max-y) (bounds tick))
  (* (- max-x min-x)
     (- max-y min-y)))

; Find the tick where volume hits a minimum
(define (min-volume)
  (let loop ([v (volume 0)]
             [tick 1])
    (define v^ (volume tick))
    (cond
      [(> v^ v) (sub1 tick)]
      [else
       (loop v^ (add1 tick))])))
  
; Render a given ticket as an image
(define (render tick)
  (define particles (map (curryr particle-update tick) INITIAL-PARTICLES))
  (define-values (min-x max-x min-y max-y) (bounds tick))
  (flomap->bitmap
   (build-flomap*
    1
    (exact-round (+ 10 (- max-x min-x)))
    (exact-round (+ 10 (- max-y min-y)))
    (λ (x y)
      (cond
        [(for/first ([p (in-list particles)]
                     #:when (and (= x (- (particle-px p) min-x -5))
                                 (= y (- (particle-py p) min-y -5))))
           p)
         (vector 1.0)]
        [else
         (vector 0.0)])))))

; Find the minimum volume and render a few frames around that
(define minimum-tick (min-volume))
(printf "minimum volume at: ~a\n" minimum-tick)

(for ([i (in-range (- minimum-tick 10) (+ minimum-tick 11))])
  (printf "rendering ~a\n" i)
  (send (render i)
        save-file
        (format "frame-~a.png" (~a i #:min-width 4 #:align 'right #:left-pad-string "0"))
        'png))
