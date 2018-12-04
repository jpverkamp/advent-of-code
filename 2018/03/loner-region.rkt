#lang racket

(require "region.rkt")

(define (overlaps? r1 r2)
  (match-define (region id1 left1 top1 width1 height1) r1)
  (match-define (region id2 left2 top2 width2 height2) r2)

  (not (or (<= (+ left1 width1) left2)
           (<= (+ left2 width2) left1)
           (<= (+ top1 height1) top2)
           (<= (+ top2 height2) top1))))

(define input-regions (read-regions))

(for*/first ([r1 (in-list input-regions)]
             #:unless (for/first ([r2 (in-list input-regions)]
                                  #:when (and (not (eq? r1 r2))
                                              (overlaps? r1 r2)))
                        r2))
  r1)