import { useRef, type FC } from 'react'
import { cn } from '@/lib/utils'
import { gsap } from '@/lib/gsap'

export interface NeuSwitchProps {
  checked?: boolean
  onCheckedChange?: (checked: boolean) => void
  disabled?: boolean
  className?: string
  'aria-label'?: string
}

export const NeuSwitch: FC<NeuSwitchProps> = ({
  checked = false,
  onCheckedChange,
  disabled = false,
  className,
  'aria-label': ariaLabel = 'Toggle switch',
}) => {
  const knobRef = useRef<HTMLSpanElement | null>(null)

  const toggle = () => {
    if (disabled) return
    const nextChecked = !checked

    if (knobRef.current) {
      gsap.to(knobRef.current, {
        x: nextChecked ? 26 : 0,
        duration: 0.35,
        ease: 'back.out(1.7)',
      })
    }

    onCheckedChange?.(nextChecked)
  }

  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={ariaLabel}
      disabled={disabled}
      onClick={toggle}
      className={cn(
        'neu-inset-sm relative inline-flex h-8 w-15 shrink-0 cursor-pointer items-center p-1 transition-colors outline-none',
        'focus-visible:ring-2 focus-visible:ring-[var(--neu-blue)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--neu-bg)]',
        disabled && 'opacity-50 cursor-not-allowed',
        className
      )}
    >
      <span
        ref={knobRef}
        style={{ transform: checked ? 'translateX(26px)' : 'translateX(0px)' }}
        className={cn(
          'h-6 w-6 rounded-full transition-all duration-300',
          checked
            ? 'bg-[var(--neu-blue)] shadow-[0_0_12px_var(--neu-blue)]'
            : 'bg-[var(--neu-dark)] shadow-sm'
        )}
      />
    </button>
  )
}
