#lang racket

(provide (all-defined-out))

(struct circular-list (prev next)
  #:transparent
  #:methods gen:custom-write
  [(define write-proc
     (λ (cls out depth)
       (fprintf out "#<circular-list ~a>" (circular-list->list cls))))])

; Create an empty circular list
(define (make-circular-list)
  (circular-list '() '()))

; Turn a list into a circular list
(define (list->circular-list ls)
  (circular-list '() ls))

; Turn a circular list back into a regular one at the current point
(define (circular-list->list cls)
  (match-define (circular-list prev next) cls)
  (append next (reverse prev)))

; Test if a circular list contains no elements
(define (circular-list-empty? cls)
  (match-define (circular-list prev next) cls)
  (and (null? prev) (null? next)))

; Get size of a circular-list
(define (circular-list-length cls)
  (match-define (circular-list prev next) cls)
  (+ (length prev) (length next)))

; Look at the current first item of a circular list
(define (circular-list-peek cls)
  (match-define (circular-list prev next) cls)
  (cond
    [(null? next) (last prev)]
    [else (first next)]))

; Add a new item to the head of a circular list
(define (circular-list-push cls value)
  (match-define (circular-list prev next) cls)
  (circular-list (list* value prev) next))

; Remove the current head of a circular list
(define (circular-list-pop cls)
  (match-define (circular-list prev next) cls)
  (cond
    [(null? next)
     (define next (reverse prev))
     (circular-list '() (rest next))]
    [else
     (circular-list prev (rest next))]))

; Rotate a circular list n positions
; Positive numbers rotate 'forward', negative 'backwards'
(define (circular-list-rotate cls [steps 1])
  (match-define (circular-list prev next) cls)
  (let loop ([prev (circular-list-prev cls)]
             [next (circular-list-next cls)]
             [steps steps])
    (cond
      [(zero? steps) (circular-list prev next)]
      [(negative? steps)
       (cond
         [(null? prev)
          (define prev (reverse next))
          (loop (rest prev) (list (first prev)) (add1 steps))]
         [else
          (loop (rest prev) (list* (first prev) next) (add1 steps))])]
      [else
       (cond
         [(null? next)
          (define next (reverse prev))
          (loop (list (first next)) (rest next) (sub1 steps))]
         [else
          (loop (list* (first next) prev) (rest next) (sub1 steps))])])))

; Rotate a circular list until the head matches the given prediate
(define (circular-list-rotate-until cls pred?)
  (let loop ([length (circular-list-length cls)]
             [cls cls])
    (cond
      [(or (zero? length)
           (pred? (circular-list-peek cls)))
       cls]
      [else
       (loop (sub1 length)
             (circular-list-rotate cls -1))])))
      

