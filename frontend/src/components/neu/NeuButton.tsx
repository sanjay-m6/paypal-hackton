import { forwardRef, useRef, type ButtonHTMLAttributes } from 'react'
import { cn } from '@/lib/utils'
import { gsap } from '@/lib/gsap'

export interface NeuButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'raised' | 'inset'
  size?: 'default' | 'sm' | 'icon'
}

export const NeuButton = forwardRef<HTMLButtonElement, NeuButtonProps>(
  ({ className, variant = 'raised', size = 'default', children, ...props }, ref) => {
    const internalRef = useRef<HTMLButtonElement | null>(null)

    // Merge forwarded ref and internal ref
    const setRefs = (node: HTMLButtonElement | null) => {
      internalRef.current = node
      if (typeof ref === 'function') {
        ref(node)
      } else if (ref) {
        ref.current = node
      }
    }

    const handlePointerDown = () => {
      if (!internalRef.current) return
      gsap.to(internalRef.current, {
        scale: 0.98,
        duration: 0.15,
        ease: 'power2.out',
      })
    }

    const handlePointerUp = () => {
      if (!internalRef.current) return
      gsap.to(internalRef.current, {
        scale: 1,
        duration: 0.6,
        ease: 'elastic.out(1, 0.6)',
      })
    }

    return (
      <button
        ref={setRefs}
        onPointerDown={handlePointerDown}
        onPointerUp={handlePointerUp}
        onPointerLeave={handlePointerUp}
        className={cn(
          'inline-flex items-center justify-center font-medium text-[var(--neu-text)] outline-none transition-shadow',
          'focus-visible:ring-2 focus-visible:ring-[var(--neu-blue)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--neu-bg)]',
          'active:neu-inset disabled:pointer-events-none disabled:opacity-50 select-none cursor-pointer',
          variant === 'raised' ? (size === 'sm' ? 'neu-raised-sm' : 'neu-raised') : 'neu-inset',
          size === 'default' && 'h-12 px-6 text-sm min-w-[44px]',
          size === 'sm' && 'h-9 px-4 text-xs min-w-[36px]',
          size === 'icon' && 'h-12 w-12 p-0 min-w-[44px] min-h-[44px]',
          className
        )}
        {...props}
      >
        {children}
      </button>
    )
  }
)

NeuButton.displayName = 'NeuButton'
