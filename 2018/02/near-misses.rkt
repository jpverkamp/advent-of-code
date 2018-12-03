#lang racket

(define (differences word1 word2)
  (for/sum ([letter1 (in-string word1)]
            [letter2 (in-string word2)]
            #:unless (char=? letter1 letter2))
    1))

(define (shared-letters word1 word2)
  (list->string
   (for/list ([letter1 (in-string word1)]
              [letter2 (in-string word2)]
              #:when (char=? letter1 letter2))
     letter1)))

(define words (port->lines))

(apply shared-letters
       (for*/first ([word1 (in-list words)]
                    [word2 (in-list words)]
                    #:when (= 1 (differences word1 word2)))
         (list word1 word2)))