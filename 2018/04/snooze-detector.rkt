#lang racket

; Wrapper around in-lines to sort the values
; This is very important!
(define (in-sorted-lines [in (current-input-port)])
  (in-list (sort (port->lines) string<?)))

; Add minutes to guard `id` from `start` to `end`
; Only update if we have a valid ID
(define (add-minutes data id start end)
  (define minute-hash (hash-ref data id (hash)))
  (define updated-minute-hash
    (for/fold ([minute-hash minute-hash])
              ([minute (in-range start end)])
      (hash-update minute-hash minute add1 0)))
  (hash-set data id updated-minute-hash))

; Read guard data from stdin
(define (read-guard-data [in (current-input-port)])
  (define-values (guard-data _id _state _last-minute)
    (for/fold ([data (hash)]
               [id #f]
               [state 'awake]
               [last-minute 0])
              ([line (in-sorted-lines in)])
      (match line
        ; Switching to a new guard
        ; If we had a previous guard that was asleep, finish their shift
        ; This doesn't appear to be the case any more
        [(regexp #px"Guard #(\\d+) begins shift" (list _ raw-new-id))
         (define new-id (string->number raw-new-id))
         (define new-data (if (eq? state 'asleep) (add-minutes data id last-minute 60) data))
         (values new-data new-id 'awake 0)]
        ; Current guard fell asleep, update their state
        [(regexp #px"(\\d+)\\] falls asleep" (list _ raw-minute))
         (define minute (string->number raw-minute))
         (values data id 'asleep minute)]
        ; Current guard woke up, record their asleep time
        [(regexp #px"(\\d+)\\] wakes up" (list _ raw-minute))
         (define minute (string->number raw-minute))
         (values (add-minutes data id last-minute minute) id 'awake minute)])))
  guard-data)

; Which guard was asleep for the most minutes
(define (most-asleep-minutes data)
  (define-values (max-id max-minutes)
    (for*/fold ([max-id #f]
                [max-minutes 0])
               ([(id sleep-data) (in-hash data)]
                [minutes-asleep (in-value (apply + (hash-values sleep-data)))]
                #:when (> minutes-asleep max-minutes))
      (values id minutes-asleep)))
  max-id)

; What minute was a given guard the most asleep for
(define (sleepiest-minute data id)
  (define-values (max-minute max-value)
    (for/fold ([max-minute #f]
               [max-value 0])
              ([(minute value) (in-hash (hash-ref data id (hash)))]
               #:when (> value max-value))
      (values minute value)))
  max-minute)

; Find the sleepiest guard and his most asleep minute
(define guard-data (read-guard-data))
(define sleepiest-guard-id (most-asleep-minutes guard-data))
(define break-in-minute (sleepiest-minute guard-data sleepiest-guard-id))

(printf "[part 1] guard: ~a, minute: ~a, product: ~a\n"
        sleepiest-guard-id
        break-in-minute
        (* sleepiest-guard-id break-in-minute))

; Find the overall sleepiest guard/minute
(define-values (max-guard-id max-minute _max-value)
  (for*/fold ([max-guard-id #f]
              [max-minute #f]
              [max-value 0])
             ([(id minute-data) (in-hash guard-data)]
              [(minute value) (in-hash minute-data)]
              #:when (> value max-value))
    (values id minute value)))

(printf "[part 2] guard: ~a, minute ~a: product: ~a\n"
        max-guard-id
        max-minute
        (* max-guard-id max-minute))