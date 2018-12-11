#lang racket

(define debug (make-parameter #f))
(define base-duration (make-parameter 60))
(define workers (make-parameter 5))

(command-line
  #:once-each
 [("--debug")
  "Print debug output while solving the problem"
  (debug #t)]
 #:multi
 [("--duration")
  n
  "Base job duration (default 60)"
  (base-duration (string->number n))]
 [("--workers")
  n
  "Number of workers (default 5)"
  (workers (string->number n))])

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
; Done is a hash of node -> timestamp
(define (can-do? node done timestamp)
  (for/and ([pre (in-set (hash-ref EDGES node (set)))])
    (<= (hash-ref done pre +inf.0) timestamp)))
    
; Get a set of nodes that can be done next
(define (possible todo done timestmap)
  (for/set ([node (in-set todo)]
            #:when (can-do? node done timestmap))
    node))

; Get the lexiographically first node in a set
(define (lex-first nodes)
  (first (sort (set->list nodes) string<?)))

; Job duration is 60 + (1 for A, 2 for B, etc)
(define (duration node)
  (+ 1
     (base-duration)
     (char->integer (string-ref node 0))
     (- (char->integer #\A))))

; Main body: Simulate multiple jobs running at once
(printf "[part2] ")
(newline)
(let loop ([todo NODES]   ; Set of nodes to work on
           [done (hash)]  ; Hash of node -> time finished
           [timestamp 0]  ; Current timestamp 
           [workers (for/hash ([i (in-range (workers))])
                      (values i 0))])

  ; Pre-calculate any workers/jobs that became free this tick
  (define free-workers
    (for/list ([(id finished-timestamp) (in-hash workers)]
                #:when (<= finished-timestamp timestamp))
       id))

  (define free-jobs
    (possible todo done timestamp))
  
  (cond
    ; Base case, work is done
    ; Return nodes sorted first by finish time then by lex
    [(set-empty? todo)
     (apply max (hash-values done))]
    ; No workers/jobs are free, just advance one tick
    [(or (null? free-workers) (set-empty? free-jobs))
     (loop todo done (add1 timestamp) workers)]
    ; We have work to do and at least one worker is first
    ; Assign one worker at a time, don't advance time
    [else
     (define next (lex-first free-jobs))
     (define next-time (+ timestamp (duration next)))

     (when (debug)
       (printf "[~a] ~a started ~a (~a sec)\n"
               timestamp
               (first free-workers)
               next
               (duration next)))
     
     (loop (set-remove todo next)
           (hash-set done next next-time)
           timestamp
           (hash-set workers (first free-workers) next-time))]))