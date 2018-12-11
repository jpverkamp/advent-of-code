#lang racket

(define-values (EDGES NODES)
  (for/fold ([edges (hash)]
             [nodes (set)])
            ([line (in-lines)])
    
    (define parts (string-split line))
    (define pre (list-ref parts 1))
    (define post (list-ref parts 7))

    (values (hash-update edges
                         post
                         (curryr set-add pre)
                         (set))
            (set-add (set-add nodes pre) post))))

; Tests if a given node can be done right now given already done nodes
(define (can-do? node done)
  (for/and ([pre (in-set (hash-ref EDGES node (set)))])
    (set-member? done pre)))

; Get a set of nodes that can be done next
(define (possible todo done)
  (for/set ([node (in-set todo)]
            #:when (can-do? node done))
    node))

; Get the lexiographically first node in a set
(define (lex-first nodes)
  (first (sort (set->list nodes) string<?)))

; Main body: do nodes one at a time
; Each time take the lexiographically first node that has no more dependencies
(printf "[part1] ")
(apply string-append 
       (let loop ([todo NODES]
                  [done (set)]
                  [order (list)])
         (cond
           ; Base case, return order
           [(set-empty? todo) (reverse order)]
           ; Otherwise, find the next node
           [else
            (define next (lex-first (possible todo done)))
            (loop (set-remove todo next)
                  (set-add done next)
                  (list* next order))])))

