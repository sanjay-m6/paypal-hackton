import { useRef, useState, type FC } from 'react'
import { Heart } from 'lucide-react'
import { cn } from '@/lib/utils'
import { gsap } from '@/lib/gsap'

export interface NeuLikeButtonProps {
  initialLiked?: boolean
  variant?: 'raised' | 'inset'
  onToggle?: (liked: boolean) => void
  className?: string
  'aria-label'?: string
}

export const NeuLikeButton: FC<NeuLikeButtonProps> = ({
  initialLiked = false,
  variant: controlledVariant,
  onToggle,
  className,
  'aria-label': ariaLabel = 'Like button',
}) => {
  const [liked, setLiked] = useState(initialLiked)
  const heartRef = useRef<SVGSVGElement | null>(null)
  const containerRef = useRef<HTMLButtonElement | null>(null)

  const isLiked = initialLiked !== undefined ? liked : false
  const effectiveVariant = controlledVariant ?? (isLiked ? 'inset' : 'raised')

  const handleClick = () => {
    const nextLiked = !isLiked
    setLiked(nextLiked)

    if (heartRef.current) {
      gsap.fromTo(
        heartRef.current,
        { scale: 1 },
        {
          scale: 1.35,
          duration: 0.45,
          ease: 'elastic.out(1.2, 0.4)',
          yoyo: true,
          repeat: 1,
        }
      )
    }

    onToggle?.(nextLiked)
  }

  return (
    <button
      ref={containerRef}
      type="button"
      aria-label={ariaLabel}
      aria-pressed={isLiked}
      onClick={handleClick}
      className={cn(
        'inline-flex h-20 w-20 cursor-pointer items-center justify-center rounded-3xl transition-all duration-300 outline-none',
        'focus-visible:ring-2 focus-visible:ring-[var(--neu-blue)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--neu-bg)]',
        effectiveVariant === 'inset' ? 'neu-inset' : 'neu-raised',
        className
      )}
    >
      <Heart
        ref={heartRef}
        className={cn(
          'h-8 w-8 transition-colors duration-300',
          isLiked || controlledVariant === 'inset'
            ? 'fill-[var(--neu-red)] text-[var(--neu-red)] filter drop-shadow-[0_0_8px_var(--neu-red)]'
            : 'fill-[var(--neu-text)] text-[var(--neu-text)]'
        )}
      />
    </button>
  )
}
