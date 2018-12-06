#lang racket

; Two units react if they are the same type (letter) and opposite polarity (case)
(define (react? a b)
  (and (not (eq? a b))
       (eq? (char-downcase a)
            (char-downcase b))))

; Collapse all unit pairs that can one time
(define (collapse-once ls)
  (match ls
    ; If The first two units react, remove them
    [(list-rest a b rest)
     #:when (react? a b)
     (collapse-once rest)]
    ; Otherwise, if we have at least one item, process the rest
    [(list-rest a rest)
     (list* a (collapse-once rest))]
    ; Base case: nothing left
    [else
     ls]))

; Collapse until there's nothing more to do
(define (collapse ls)
  (let ([ls^ (collapse-once ls)])
    (if (equal? ls ls^)
        ls
        (collapse ls^))))

(define input-polymer (string->list (read-line)))
(define output-polymer (collapse input-polymer))
(printf "[part 1] output length: ~a\n" (length output-polymer))

; Remove all instances of a unit (either polarity)
(define (remove/ignore-case ls c)
  (filter (λ (a) (not (eq? (char-downcase a) (char-downcase c)))) ls))

; Try removing each letter in turn, recording each new best
(define-values (best-to-remove best-length)
  (for*/fold ([best-to-remove #f]
              [best-length +inf.0])
             ([to-remove (in-string "abcdefghijklmnopqrstuvwxyz")]
              [length (in-value (length (collapse (remove/ignore-case input-polymer to-remove))))]
              #:when (< length best-length))
    (values to-remove length)))

(printf "[part 2] removing ~a gives a length of: ~a\n" best-to-remove best-length)
