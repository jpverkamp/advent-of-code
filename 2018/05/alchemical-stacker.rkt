#lang racket

; Two units react if they are the same type (letter) and opposite polarity (case)
(define (react? a b)
  (and (not (eq? a b))
       (eq? (char-downcase a)
            (char-downcase b))))

; Collapse an alchemical polymer removing matching units of opposite polarity
(define (collapse polymer)
  (let loop ([input polymer]
             [output '()])
    (cond
      ; End condition, output the stack
      [(null? input) (reverse output)]
      ; Initial state, nothing on the output stack to compare
      [(null? output)
       (loop (rest input) (list (first input)))]
      ; Top of stack and next of input match, remove both
      ; This will allow chain reactions since it exposes a new top of stack to react
      [(react? (first input) (first output))
       (loop (rest input) (rest output))]
      ; Don't react, move to output
      [else
       (loop (rest input) (list* (first input) output))])))

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
