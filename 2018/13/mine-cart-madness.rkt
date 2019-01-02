#lang racket

(require images/flomap
         racket/cmdline)

(define debug-print (make-parameter #f))
(define debug-frames (make-parameter #f))
(define current-frame (make-parameter 0))

(command-line
 #:once-each
 [("--debug") "Print debug output while solving the problem" (debug-print #t)]
 [("--debug-frames") "Render frames of progress as pngs" (debug-frames #t)])

(struct point (x y) #:transparent)
(struct cart (location velocity next-turn) #:transparent)
(struct track (data carts top-left bottom-right) #:transparent)

; Read a track from the current-input
(define (read-track [in (current-input-port)])
  ; Read the raw data
  (define raw-data
    (for/fold ([track (hash)])
              ([line (in-lines in)]
               [y (in-naturals)])
      (for/fold ([track track])
                ([c (in-string line)]
                 [x (in-naturals)]
                 #:unless (equal? c #\space))
        (hash-set track (point x y) c))))
  
  ; For each cart character, track to underlying track and current velocity
  (define cart-track-data
    (hash #\> (list #\-  1 0)
          #\< (list #\- -1 0)
          #\^ (list #\| 0 -1)
          #\v (list #\| 0  1)))

  ; Determine where the carts are
  (define-values (data carts)
    (for*/fold ([data raw-data]
                [carts (list)])
               ([(p c) (in-hash raw-data)]
                [ctd (in-value (hash-ref cart-track-data c #f))]
                #:when ctd)
      (values
       ; Overwrite the track point with the underlying track
       (hash-set data p (first ctd))
       ; Store the cart position and velocity 
       (list* (cart p (point (second ctd) (third ctd)) 'left) carts))))

  ; Determine the bounds for the track
  (define-values (min-x max-x min-y max-y)
    (for/fold ([min-x #f] [max-x #f] [min-y #f] [max-y #f])
              ([(p c) (in-hash data)])
      (values (min (or min-x (point-x p)) (point-x p))
              (max (or max-x (point-x p)) (point-x p))
              (min (or min-y (point-y p)) (point-y p))
              (max (or max-y (point-y p)) (point-y p)))))

  ; Finally create the track structure
  (track data
         carts
         (point min-x min-y)
         (point max-x max-y)))

; Helper function to display a track
(define (display-track m)
  (for ([y (in-range (point-y (track-top-left m)) (add1 (point-y (track-bottom-right m))))])
    (for ([x (in-range (point-x (track-top-left m)) (add1 (point-x (track-bottom-right m))))])
      (cond
        [(for/first ([c (in-list (track-carts m))]
                     #:when (equal? (cart-location c) (point x y)))
           c)
         => (λ (c)
              (display
               (match (cart-velocity c)
                 [(point 0  1) #\v]
                 [(point 0 -1) #\^]
                 [(point  1 0) #\>]
                 [(point -1 0) #\<])))]
        [else
         (display (hash-ref (track-data m) (point x y) #\.))]))
    (newline)))

; Helper function to render a map as a frame to an image with ASCII graphics
(define (render-frame m prefix)
  (match-define (track data carts (point left top) (point right bottom)) m)
  (printf "rendering frame ~a:~a\n" prefix (current-frame))
  (send
   (flomap->bitmap
    (build-flomap*
     3 (add1 (- right left)) (add1 (- bottom top))
     (λ (x y)
       (cond
         ; Carts are red
         [(for/first ([c (in-list carts)]
                      #:when (equal? (point x y) (cart-location c)))
            c)
          (vector 1 0 0)]
         ; Tracks are white
         [(hash-ref data (point x y) #f)
          (vector 1 1 1)]
         ; Background is black
         [else
          (vector 0 0 0)]))))
   save-file
   (~a "frame-" prefix "-" (~a (current-frame) #:min-width 4 #:left-pad-string "0" #:align 'right) ".png")
   'png)
  (current-frame (add1 (current-frame))))

; Allow sorting a list of carts by location top to bottom, left to right
(define (cart-location-<? c1 c2)
  (or (< (point-y (cart-location c1))
         (point-y (cart-location c2)))
      (and (= (point-y (cart-location c1))
              (point-y (cart-location c2)))
           (< (point-x (cart-location c1))
              (point-x (cart-location c2))))))

; Update a track by one tick
; If two carts collide, an exception will be raised, catch it and return the cart
(define (update-track m)
  ; Update carts, has to be done oddly since they can collide half way through an update
  ; Otherwise, remove the third argument from update-cart and use:
  ;   (map (curry update-cart m) (sort (track-carts m) cart-location-<?))
  (define updated-carts
    (let loop ([done '()]
               [todo (sort (track-carts m) cart-location-<?)])
      (cond
        [(null? todo) done]
        [else
         ; TODO: Make this more efficient than using append
         (loop (list* (update-cart m (first todo) (append done todo)) done)
               (rest todo))])))

  (track (track-data m)
         updated-carts
         (track-top-left m)
         (track-bottom-right m)))

; Update a cart by one tick
; Return the new cart; but if it collided raise the cart as an exception insteadcurrent-frame
; c* Is a running updated list of carts, since collisions can happen with already moved carts
(define (update-cart m c c*)
  (match-define (cart (point x y) (point vx vy) rotation) c)
  
  (define new-location (point (+ x vx) (+ y vy)))
  
  (define-values (new-velocity new-rotation)
    (case (hash-ref (track-data m) new-location)
      [(#\/) (values (point (- vy) (- vx)) rotation)]
      [(#\\) (values (point vy vx) rotation)]
      [(#\+)
       (case rotation
         [(left)     (values (point vy (- vx)) 'straight)]
         [(straight) (values (point vx vy)     'right)]
         [(right)    (values (point (- vy) vx) 'left)])]
      [else  (values (point vx vy) rotation)]))
  
  (define updated-cart (cart new-location new-velocity new-rotation))

  ; Check for collisions
  (for ([c (in-list c*)]
        #:when (equal? new-location (cart-location c)))
    (raise updated-cart))

  updated-cart)

; Run a track until collision, return the cart that collided
(define (update-track-until-collision m)
  (with-handlers ([cart? identity])
    (let loop ([m m])
      (when (debug-print) (display-track m) (newline))
      (when (debug-frames) (render-frame m "collision"))
      
      (loop (update-track m)))))
    
; Run the main program
(printf "[part1]\n")
(define input (read-track))
(update-track-until-collision input)

; Update a track by one tick
; If two carts collide, remove those two carts and contiue updating
(define (update-track/remove-collisions m)
  ; Helper to remove carts at a given location from a list
  (define (remove-by-location carts location)
    (for/list ([c (in-list carts)]
               #:unless (equal? (cart-location c) location))
      c))

  ; Calculate the new list of carts, some might collide
  (define updated-carts
    (let loop ([done '()]
               [todo (sort (track-carts m) cart-location-<?)])
      (cond
        [(null? todo) done]
        [else
         (with-handlers
             ; Case where carts collided
             ; Remove that cart + all other carts with that coordinate from both lists
             ([cart? (λ (c)
                       (loop (remove-by-location done (cart-location c))
                             (remove-by-location (rest todo) (cart-location c))))])
           ; Otherwise
           (loop (list* (update-cart m (first todo) (append done todo)) done)
                 (rest todo)))])))

  (track (track-data m)
         updated-carts
         (track-top-left m)
         (track-bottom-right m)))

; Run a track until there is only one cart left, removing carts that collide
(define (update-track-until-singleton m)
  (let loop ([m m])
    (when (debug-print) (display-track m) (newline))
    (when (debug-frames) (render-frame m "singleton"))
    
    (cond
      [(<= (length (track-carts m)) 1)
       (first (track-carts m))]
      [else
       (loop (update-track/remove-collisions m))])))

; Run the main program again
(printf "\n[part2]\n")
(current-frame 0)
(update-track-until-singleton input)