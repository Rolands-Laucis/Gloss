<script>
    export let classes='', style='', title='',
                inert=false, tabindex=-1,
                size = 14, fill=0, wgth=200, grad=0, opsz=24,
                hover_fill=false, to_fill = parseInt(fill) === 0 ? 1 : 0, to_wgth = wgth, to_grad = grad, to_opsz = opsz;
</script>
<!-- https://developers.google.com/fonts/docs/material_symbols#use_in_web -->
<!-- https://fonts.google.com/knowledge/glossary/grade_axis -->
<!-- https://fonts.google.com/knowledge/glossary/optical_size_axis -->

<!-- class="material-icons-round {classes}"  -->
<span 
    class="material-symbols-round {classes}" 
    class:inert
    class:hover_fill
    style="font-size:{size}px;--f:{fill};--f_h:{to_fill}; --w:{wgth};--w_h:{to_wgth}; --g:{grad};--g_h:{to_grad}; --o:{opsz};--o_h:{to_opsz};{style}"
    {title}
    {tabindex}
    on:click|stopPropagation>
    <slot></slot>
</span>

<style lang="scss">
    span{
        user-select: none;
        cursor: inherit;
        font-variation-settings: 'FILL' var(--f), 'wght' var(--w), 'GRAD' var(--g), 'opsz' var(--o);
        
        &:hover, &:focus{
            cursor: pointer;
            --w: var(--w_h) !important;
            --g: var(--g_h) !important;
            --o: var(--o_h) !important;
            filter: brightness(1.5);
        }
        &:focus{
            outline: 2px solid $l;
            border-radius: $s-03;
        }
        &:is(.hover_fill):hover{
            --f: var(--f_h) !important;
        }
        
        &.inert{pointer-events: none;}
    }

    @keyframes anim_shake {
        0%{rotate: 0deg;}
        5%{rotate: -12deg;}
        20%{rotate: 12deg;}
        40%{rotate: -6deg;}
        60%{rotate: 6deg;}
        80%{rotate: -3deg;}
        100%{rotate: 0deg;}
    }
    .anim_shake{
        animation: anim_shake 350ms ease-in forwards;
    }
</style>