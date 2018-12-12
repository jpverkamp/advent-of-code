#lang racket

(struct tree (children metadata) #:transparent)

; A tree is count-children, count-metadata, children..., metadata...
(define (read-tree)
  (define child-count (read))
  (define metadata-count (read))
  (tree (for/list ([i (in-range child-count)])
          (read-tree))
        (for/list ([i (in-range metadata-count)])
          (read))))

; Sum all metadata values for a simple checksum
(define (simple-checksum tr)
  (+ (for/sum ([child (in-list (tree-children tr))])
       (simple-checksum child))
     (apply + (tree-metadata tr))))

; Checksum with no children is sum of metadata
; Checksum with children uses the metadata as index, sums those children
(define (complex-checksum tr)
  (cond
    [(null? (tree-children tr))
     (apply + (tree-metadata tr))]
    [else
     (for/sum ([index (in-list (tree-metadata tr))])
       (cond
         [(<= 1 index (length (tree-children tr)))
          (complex-checksum (list-ref (tree-children tr) (sub1 index)))]
         [else
          0]))]))
  
(define TREE (read-tree))

(printf "[part 1] ~a\n" (simple-checksum TREE))
(printf "[part 2] ~a\n" (complex-checksum TREE))
